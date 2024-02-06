use crate::resolvers::{
    cane_library::{CaneLibraryMutation, CaneLibraryQuery},
    cane_mount::{CaneMountMutation, CaneMountQuery},
    crystal::{CrystalMutation, CrystalQuery},
    pin_library::{PinLibraryMutation, PinLibraryQuery},
    pin_mount::{PinMountMutation, PinMountQuery},
    puck_library::{PuckLibraryMutation, PuckLibraryQuery},
    puck_mount::{PuckMountMutation, PuckMountQuery},
};
use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};

pub fn root_schema_builder() -> SchemaBuilder<Query, RootMutation, EmptySubscription> {
    Schema::build(
        Query::default(),
        RootMutation::default(),
        EmptySubscription,
    )
    .enable_federation()
}

pub type RootSchema = Schema<Query, RootMutation, EmptySubscription>;

#[derive(Debug, Clone, MergedObject, Default)]
pub struct Query(
    CrystalQuery,
    CaneLibraryQuery,
    CaneMountQuery,
    PuckLibraryQuery,
    PuckMountQuery,
    PinLibraryQuery,
    PinMountQuery,
);

#[derive(Debug, Clone, MergedObject, Default)]
pub struct RootMutation(
    CrystalMutation,
    CaneLibraryMutation,
    CaneMountMutation,
    PuckLibraryMutation,
    PuckMountMutation,
    PinLibraryMutation,
    PinMountMutation,
);
