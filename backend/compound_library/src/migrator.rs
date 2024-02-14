// src/migrator.rs

use crate::entities::compound_library;
use axum::async_trait;
use sea_orm::{DbErr, DeriveMigrationName, Schema};
use sea_orm_migration::{MigrationTrait, MigratorTrait, SchemaManager};

pub struct Migrator;

#[derive(DeriveMigrationName)]
struct Initial;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(Initial)]
    }
}

#[async_trait]
impl MigrationTrait for Initial {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let backend = manager.get_database_backend();
        let schema = Schema::new(backend);

        manager
            .create_type(schema.create_enum_from_active_enum::<compound_library::CompoundState>())
            .await?;

        manager
            .create_table(schema.create_table_from_entity(compound_library::Entity))
            .await?;

        Ok(())
    }
}