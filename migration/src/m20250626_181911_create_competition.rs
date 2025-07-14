use sea_orm_migration::{prelude::{extension::postgres::{Type, TypeDropStatement}, *}, schema::{pk_auto, string, string_null}};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                .as_enum(CompetitionTypes::Enum)
                .values([
                    CompetitionTypes::FirstTier,
                    CompetitionTypes::SecondTier,
                    CompetitionTypes::ThirdTier,
                    CompetitionTypes::FourthTier,
                    CompetitionTypes::FifthTier,
                    CompetitionTypes::SixthTier,
                    CompetitionTypes::YouthLeague,
                    CompetitionTypes::DomesticCup,
                    CompetitionTypes::DomesticSuperCup,
                    CompetitionTypes::PlayOffs,
                    CompetitionTypes::LeagueCup,
                    CompetitionTypes::DomesticYouthCup,
                    CompetitionTypes::ReserveLeague,
                    CompetitionTypes::FurtherCup,
                    CompetitionTypes::NationalYouthSuperCup,
                    CompetitionTypes::InternationalCups,
                    CompetitionTypes::Cups,
                ])
                .to_owned()
            ).await?;

        manager
            .create_table(
                Table::create()
                    .table(Competitions::Table)
                    .if_not_exists()
                    .col(pk_auto(Competitions::Id))
                    .col(string(Competitions::TmId))
                    .col(string(Competitions::Name))
                    .col(string(Competitions::Country))
                    .col(ColumnDef::new(Competitions::CompetitionType).custom(CompetitionTypes::Enum).not_null())
                    .col(string_null(Competitions::Slug))
                    .to_owned()
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-tm_id")
                    .table(Competitions::Table)
                    .col(Competitions::TmId)
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
                    .col(string(Seasons::Label))
                    .to_owned()
            )
            .await?;    
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {     
        manager.drop_index(Index::drop().name("idx-tm_id").to_owned())
        .await?;
        manager.drop_table(Table::drop().table(Seasons::Table).if_exists().to_owned())
        .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(Competitions::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_type(
                TypeDropStatement::new()
                    .name(CompetitionTypes::Enum)
                    .if_exists()
                    .to_owned()
            )
            .await?;   
        
    Ok(())
    }
}

#[derive(DeriveIden)]
enum Competitions {
    Table,
    Id,
    #[sea_orm(iden = "tm_id")]
    TmId,
    Name,
    CompetitionType,
    Country,
    Slug
}

#[derive(DeriveIden)]
enum Seasons {
    Table,
    Id,
    Label
}

#[derive(DeriveIden)]
enum CompetitionTypes {
    Enum,
    #[sea_orm(iden = "First Tear")]
    FirstTier,
    #[sea_orm(iden = "Second Tear")]
    SecondTier,
    #[sea_orm(iden = "Third Tear")]
    ThirdTier,
    #[sea_orm(iden = "Fourth Tear")]
    FourthTier,
    #[sea_orm(iden = "Fifth Tear")]
    FifthTier,
    #[sea_orm(iden = "Sixth Tear")]
    SixthTier,
    #[sea_orm(iden = "Youth League")]
    YouthLeague,
    #[sea_orm(iden = "Domestic Cup")]
    DomesticCup,
    #[sea_orm(iden = "Domestic Super Cup")]
    DomesticSuperCup,
    #[sea_orm(iden = "Play-Offs")]
    PlayOffs,
    #[sea_orm(iden = "League Cup")]
    LeagueCup,
    #[sea_orm(iden = "Domestic Youth Cup")]
    DomesticYouthCup,
    #[sea_orm(iden = "Reserve league")]
    ReserveLeague,
    #[sea_orm(iden = "Further Cup")]
    FurtherCup,
    #[sea_orm(iden = "National Youth Super Cup")]
    NationalYouthSuperCup,
    #[sea_orm(iden = "International Cups")]
    InternationalCups,
    #[sea_orm(iden = "Cups")]
    Cups,
}