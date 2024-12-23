use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

use super::m20241216_123238_acstor_storage_types::StorageTypes;
use super::m20241223_095243_acstor_features::AcstorFeatures;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum AcstorFeaturesToStorageTypes {
    Table,
    FeatureId,
    StorageTypeId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(AcstorFeaturesToStorageTypes::Table)
                    .col(integer(AcstorFeaturesToStorageTypes::FeatureId))
                    .col(integer(AcstorFeaturesToStorageTypes::StorageTypeId))
                    .primary_key(
                        Index::create()
                            .col(AcstorFeaturesToStorageTypes::FeatureId)
                            .col(AcstorFeaturesToStorageTypes::StorageTypeId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-type-acstor-feature")
                            .from(
                                AcstorFeaturesToStorageTypes::Table,
                                AcstorFeaturesToStorageTypes::FeatureId,
                            )
                            .to(AcstorFeatures::Table, AcstorFeatures::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-type-acstor-storage-type")
                            .from(
                                AcstorFeaturesToStorageTypes::Table,
                                AcstorFeaturesToStorageTypes::StorageTypeId,
                            )
                            .to(StorageTypes::Table, StorageTypes::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(AcstorFeaturesToStorageTypes::Table)
                    .to_owned(),
            )
            .await
    }
}
