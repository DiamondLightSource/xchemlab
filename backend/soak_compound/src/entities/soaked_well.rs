// src/entities/soaked.rs

use super::{compound_library, well_library};
use async_graphql::SimpleObject;
use sea_orm::prelude::*;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[graphql(complex, name = "soaked")]
#[sea_orm(table_name = "soaked")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment=false)]
    pub id: Uuid,
    pub well_id: Uuid,
    pub compound_id: Uuid,
    #[sea_orm(column_type = "Double")]
    pub volume: f64,
}

#[derive(Clone, Copy, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "compound_library::Entity",
        from = "Column::CompoundId",
        to = "compound_library::Column::Id"
    )]
    Compounds,
    #[sea_orm(
        belongs_to = "well_library::Entity",
        from = "Column::WellId",
        to = "well_library::Column::Id"
    )]
    Wells,
}

impl Related<compound_library::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Compounds.def()
    }
}

impl Related<well_library::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Wells.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}