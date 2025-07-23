use crate::{entity::sea_orm_active_enums::Enum, models::competition::CompetitionTypes};

pub fn map_competition_type(ct: CompetitionTypes) -> Enum {
    match ct {
        CompetitionTypes::FirstTier             => Enum::FirstTier,
        CompetitionTypes::SecondTier            => Enum::SecondTier,
        CompetitionTypes::ThirdTier             => Enum::ThirdTier,
        CompetitionTypes::FourthTier            => Enum::FourthTier,
        CompetitionTypes::FifthTier             => Enum::FifthTier,
        CompetitionTypes::SixthTier             => Enum::SixthTier,
        CompetitionTypes::YouthLeague           => Enum::YouthLeague,
        CompetitionTypes::DomesticCup           => Enum::DomesticCup,
        CompetitionTypes::DomesticSuperCup      => Enum::DomesticSuperCup,
        CompetitionTypes::PlayOffs              => Enum::PlayOffs,
        CompetitionTypes::LeagueCup             => Enum::LeagueCup,
        CompetitionTypes::DomesticYouthCup      => Enum::DomesticYouthCup,
        CompetitionTypes::ReserveLeague         => Enum::ReserveLeague,
        CompetitionTypes::FurtherCup            => Enum::FurtherCup,
        CompetitionTypes::NationalYouthSuperCup => Enum::NationalYouthSuperCup,
        CompetitionTypes::InternationalCups     => Enum::InternationalCups,
        CompetitionTypes::Cups                  => Enum::Cups,
    }
}
