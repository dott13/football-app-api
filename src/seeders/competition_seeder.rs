use sea_orm::{sea_query::OnConflict, ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::{entity::{competitions}, models::competition::Competition, utils::competition_type_converter::map_competition_type};

pub async fn seed_competition(
    db: &DatabaseConnection,
    comps: Vec<Competition>
) -> Result<(), sea_orm::DbErr>  {
    for comp in comps {
        let am = competitions::ActiveModel {
            id: Default::default(),              
            tm_id: Set(comp.tm_id),
            name: Set(comp.name),
            country: Set(comp.country),
            slug: Set(comp.slug),
            competition_type: Set(map_competition_type(comp.competition_type)),
            ..Default::default()
        };

        competitions::Entity::insert(am)
          .on_conflict(
              // assuming `tm_id` is unique
              OnConflict::column(competitions::Column::TmId)
                  .update_columns([
                      competitions::Column::Name,
                      competitions::Column::Country,
                      competitions::Column::Slug,
                      competitions::Column::CompetitionType
                  ])
                  .to_owned()
          )
          .exec(db)
          .await?;
    }

    Ok(())
} 