use sea_orm_migration::{prelude::*, schema::{integer, pk_auto, string, string_null}};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Leagues::Table)
                    .if_not_exists()
                    .col(pk_auto(Leagues::Id))
                    .col(string(Leagues::TmId))
                    .col(string(Leagues::Name))
                    .col(string_null(Leagues::Slug))
                    .to_owned()
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-tm_id")
                    .table(Leagues::Table)
                    .col(Leagues::TmId)
                    .unique()
                    .to_owned()
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Seasons::Table)
                    .if_not_exists()
                    .col(pk_auto(Seasons::Id))
                    .col(integer(Seasons::LeagueId))
                    .col(string(Seasons::Label))
                    .to_owned()
            )
            .await?;    

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-seasons-league_id")
                    .from_tbl(Seasons::Table)
                    .from_col(Seasons::LeagueId)
                    .to_tbl(Leagues::Table)
                    .to_col(Leagues::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned()
            ).await?;    
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_foreign_key(ForeignKey::drop().name("fk-seasons-league_id").to_owned()).await?;
        manager.drop_table(Table::drop().table(Seasons::Table).if_exists().to_owned())
        .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(Leagues::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        
        manager.drop_index(Index::drop().name("idx-tm_id").to_owned())
        .await?;

    Ok(())
    }
}

#[derive(DeriveIden)]
enum Leagues {
    Table,
    Id,
    #[sea_orm(iden = "tm_id")]
    TmId,
    Name,
    Slug
}

#[derive(DeriveIden)]
enum Seasons {
    Table,
    Id,
    LeagueId,
    Label
}
