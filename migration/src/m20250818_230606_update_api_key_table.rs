use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ApiKey::Table)
                    .rename_column("key", "key_hash")
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ApiKey::Table)
                    .rename_column("key_hash", "key")
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum ApiKey {
    Table,
}
