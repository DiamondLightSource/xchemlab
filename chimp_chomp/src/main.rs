#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![doc=include_str!("../README.md")]

/// Utilities for loading images.
mod image_loading;
/// Neural Netowrk inference with [`ort`].
mod inference;
/// RabbitMQ [`Job`] queue consumption and [`Response`] publishing.
mod jobs;
/// Neural Network inference postprocessing with optimal insertion point finding.
mod postprocessing;
/// Well localisation.
mod well_centering;

use crate::{
    inference::{inference_worker, setup_inference_session},
    jobs::{
        consume_job, produce_error, produce_response, setup_job_consumer, setup_rabbitmq_client,
    },
    postprocessing::inference_postprocessing,
    well_centering::well_centering,
};
use chimp_protocol::{Circle, Job};
use clap::Parser;
use futures::future::Either;
use futures_timer::Delay;
use postprocessing::Contents;
use std::{collections::HashMap, time::Duration};
use tokio::{select, spawn, task::JoinSet};
use url::Url;

/// An inference worker for the Crystal Hits in My Plate (CHiMP) neural network.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    /// The URL of the RabbitMQ server.
    rabbitmq_url: Url,
    /// The RabbitMQ channel on which jobs are assigned.
    rabbitmq_channel: String,
    /// The duration (in milliseconds) to wait after completing all jobs before shutting down.
    #[arg(long, env)]
    timeout: Option<u64>,
    /// The number of worker threads to use
    #[arg(long, env)]
    threads: Option<usize>,
}

fn main() {
    dotenvy::dotenv().ok();
    let args = Cli::parse();
    opencv::core::set_num_threads(0).unwrap();

    let runtime = {
        let mut builder = tokio::runtime::Builder::new_multi_thread();
        builder.enable_all();
        if let Some(threads) = args.threads {
            builder.worker_threads(threads);
        }
        builder.build().unwrap()
    };
    runtime.block_on(run(args));
}

#[allow(clippy::missing_docs_in_private_items)]
async fn run(args: Cli) {
    let session = setup_inference_session().unwrap();
    let input_width = session.inputs[0].dimensions[3].unwrap();
    let input_height = session.inputs[0].dimensions[2].unwrap();
    let batch_size = session.inputs[0].dimensions[0].unwrap().try_into().unwrap();

    let rabbitmq_client = setup_rabbitmq_client(args.rabbitmq_url).await.unwrap();
    let job_channel = rabbitmq_client.create_channel().await.unwrap();
    let response_channel = rabbitmq_client.create_channel().await.unwrap();
    let job_consumer = setup_job_consumer(job_channel, args.rabbitmq_channel)
        .await
        .unwrap();

    let (chimp_image_tx, chimp_image_rx) = tokio::sync::mpsc::channel(batch_size);
    let (well_image_tx, mut well_image_rx) = tokio::sync::mpsc::unbounded_channel();
    let (well_location_tx, mut well_location_rx) =
        tokio::sync::mpsc::unbounded_channel::<(Circle, Job)>();
    let (prediction_tx, mut prediction_rx) = tokio::sync::mpsc::unbounded_channel();
    let (contents_tx, mut contents_rx) = tokio::sync::mpsc::unbounded_channel::<(Contents, Job)>();
    let (error_tx, mut error_rx) = tokio::sync::mpsc::unbounded_channel();

    spawn(inference_worker(
        session,
        batch_size,
        chimp_image_rx,
        prediction_tx,
    ));

    let mut tasks = JoinSet::new();

    let mut well_locations = HashMap::new();
    let mut well_contents = HashMap::new();

    loop {
        let timeout = if let Some(timeout) = args.timeout {
            Either::Left(Delay::new(Duration::from_millis(timeout)))
        } else {
            Either::Right(std::future::pending())
        };

        select! {
            biased;

            Some((error, job)) = error_rx.recv() => {
                tasks.spawn(produce_error(error, job, response_channel.clone()));
            }

            Some((well_location, job)) = well_location_rx.recv() => {
                if let Some(contents) = well_contents.remove(&job.id) {
                    tasks.spawn(produce_response(contents, well_location, job, response_channel.clone()));
                } else {
                    well_locations.insert(job.id, well_location);
                }
            }

            Some((contents, job)) = contents_rx.recv() => {
                if let Some(well_location) = well_locations.remove(&job.id) {
                    tasks.spawn(produce_response(contents, well_location, job, response_channel.clone()));
                } else {
                    well_contents.insert(job.id, contents);
                }
            }

            chimp_permit = chimp_image_tx.clone().reserve_owned() => {
                let chimp_permit = chimp_permit.unwrap();
                tasks.spawn(consume_job(job_consumer.clone(), input_width, input_height, chimp_permit, well_image_tx.clone(), error_tx.clone()));
            }

            Some((well_image, job)) = well_image_rx.recv() =>  {
                tasks.spawn(well_centering(well_image, job, well_location_tx.clone(), error_tx.clone()));
            }

            Some((bboxes, labels, _, masks, job)) = prediction_rx.recv() => {
                tasks.spawn(inference_postprocessing(bboxes, labels, masks, job, contents_tx.clone(), error_tx.clone()));
            }

            _ = timeout => {
                println!("Stopping: No jobs processed for {}ms", args.timeout.unwrap());
                break;
            }

            else => break
        }
    }
}
