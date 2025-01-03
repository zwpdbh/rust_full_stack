//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "acstor_features_to_storage_types")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key, auto_increment = false)]
    pub feature_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub storage_type_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::acstor_features::Entity",
        from = "Column::FeatureId",
        to = "super::acstor_features::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    AcstorFeatures,
    #[sea_orm(
        belongs_to = "super::storage_types::Entity",
        from = "Column::StorageTypeId",
        to = "super::storage_types::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    StorageTypes,
}

impl Related<super::acstor_features::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcstorFeatures.def()
    }
}

impl Related<super::storage_types::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StorageTypes.def()
    }
}
