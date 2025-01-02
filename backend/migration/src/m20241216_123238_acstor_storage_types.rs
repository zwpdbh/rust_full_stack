use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
pub enum StorageTypes {
    Table,
    Id,
    Name,
    Description,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(StorageTypes::Table)
                    .col(pk_auto(StorageTypes::Id))
                    .col(string(StorageTypes::Name))
                    .col(string_null(StorageTypes::Description))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StorageTypes::Table).to_owned())
            .await
    }
}
