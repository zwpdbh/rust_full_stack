use super::m20241113_065545_posts::Posts;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Files {
    Table,
    Id,
    PostId,
    FilePath,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Files::Table)
                    .col(pk_auto(Files::Id))
                    .col(integer(Files::PostId))
                    .col(string(Files::FilePath))
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_files_posts_id")
                            .from(Files::Table, Files::PostId)
                            .to(Posts::Table, Posts::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Files::Table).to_owned())
            .await
    }
}
