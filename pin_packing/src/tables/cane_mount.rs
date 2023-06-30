use super::{cane_library, puck_mount};
use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait, Related, RelationTrait,
};

pub const CANE_SLOTS: i16 = 7;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "cane")]
#[graphql(name = "Cane", complex)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub barcode: String,
    #[sea_orm(primary_key)]
    pub created: DateTime<Utc>,
    pub operator_id: String,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "cane_library::Entity",
        from = "Column::Barcode",
        to = "cane_library::Column::Barcode"
    )]
    LibraryCane,
    #[sea_orm(has_many = "puck_mount::Entity")]
    PuckMount,
}

impl Related<cane_library::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::LibraryCane.def()
    }
}

impl Related<puck_mount::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::PuckMount.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
