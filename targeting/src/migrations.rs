use axum::async_trait;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![]
    }
}
