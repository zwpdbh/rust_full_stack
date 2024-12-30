use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(AcstorFeatures::Table)
                    .col(pk_auto(AcstorFeatures::Id))
                    .col(string(AcstorFeatures::Name))
                    .col(string_null(AcstorFeatures::Description))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AcstorFeatures::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AcstorFeatures {
    Table,
    Id,
    Name,
    Description,
}
