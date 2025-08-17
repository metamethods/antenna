use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Game::Table)
                    .if_not_exists()
                    .col(pk_auto(Game::Id))
                    .col(string(Game::Name).not_null())
                    .col(string(Game::OpenCloudApiKey).not_null())
                    .col(string(Game::UniverseId).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ApiKey::Table)
                    .if_not_exists()
                    .col(pk_auto(ApiKey::Id))
                    .col(string(ApiKey::Key).not_null().unique_key())
                    .col(integer(ApiKey::GameId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-api-key-game_id")
                            .from(ApiKey::Table, ApiKey::GameId)
                            .to(Game::Table, Game::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ApiKey::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Game::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Game {
    Table,
    Id,
    Name,
    OpenCloudApiKey,
    UniverseId,
}

#[derive(DeriveIden)]
enum ApiKey {
    Table,
    Id,
    Key,
    GameId,
}
