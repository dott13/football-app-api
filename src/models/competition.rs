use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
pub enum CompetitionTypes {
    #[serde(rename = "First Tier")]
    FirstTier,
    #[serde(rename = "Second Tier")]
    SecondTier,
    #[serde(rename = "Third Tier")]
    ThirdTier,
    #[serde(rename = "Fourth Tier")]
    FourthTier,
    #[serde(rename = "Fifth Tier")]
    FifthTier,
    #[serde(rename = "Sixth Tier")]
    SixthTier,
    #[serde(rename = "Youth League")]
    YouthLeague,
    #[serde(rename = "Domestic Cup")]
    DomesticCup,
    #[serde(rename = "Domestic Super Cup")]
    DomesticSuperCup,
    #[serde(rename = "Play-Offs")]
    PlayOffs,
    #[serde(rename = "League Cup")]
    LeagueCup,
    #[serde(rename = "Domestic Youth Cup")]
    DomesticYouthCup,
    #[serde(rename = "Reserve league")]
    ReserveLeague,
    #[serde(rename = "Further Cup")]
    FurtherCup,
    #[serde(rename = "National Youth Super Cup")]
    NationalYouthSuperCup,
    #[serde(rename = "International Cups")]
    InternationalCups,
    #[serde(rename = "Cups")]
    Cups,
}

#[derive(Serialize)]
pub struct Competition {
    pub name: String,
    pub tm_id: String,
    pub slug: Option<String>,
    pub country: String,
    pub competition_type: CompetitionTypes,
}

impl std::str::FromStr for CompetitionTypes {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "First Tier" => Ok(CompetitionTypes::FirstTier),
            "Second Tier" => Ok(CompetitionTypes::SecondTier),
            "Third Tier" => Ok(CompetitionTypes::ThirdTier),
            "Fourth Tier" => Ok(CompetitionTypes::FourthTier),
            "Fifth Tier" => Ok(CompetitionTypes::FifthTier),
            "Sixth Tier" => Ok(CompetitionTypes::SixthTier),
            "Youth League" => Ok(CompetitionTypes::YouthLeague),
            "Domestic Cup" => Ok(CompetitionTypes::DomesticCup),
            "Domestic Super Cup" => Ok(CompetitionTypes::DomesticSuperCup),
            "Play-Offs" => Ok(CompetitionTypes::PlayOffs),
            "League Cup" => Ok(CompetitionTypes::LeagueCup),
            "Domestic Youth Cup" => Ok(CompetitionTypes::DomesticYouthCup),
            "Reserve league" => Ok(CompetitionTypes::ReserveLeague),
            "Further Cup" => Ok(CompetitionTypes::FurtherCup),
            "National Youth Super Cup" => Ok(CompetitionTypes::NationalYouthSuperCup),
            "International Cups" => Ok(CompetitionTypes::InternationalCups),
            "Cups" => Ok(CompetitionTypes::Cups),
            _ => Err(())
        }
    }
}

use std::fmt;

impl fmt::Display for CompetitionTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CompetitionTypes::FirstTier             => "First Tier",
            CompetitionTypes::SecondTier            => "Second Tier",
            CompetitionTypes::ThirdTier             => "Third Tier",
            CompetitionTypes::FourthTier            => "Fourth Tier",
            CompetitionTypes::FifthTier             => "Fifth Tier",
            CompetitionTypes::SixthTier             => "Sixth Tier",
            CompetitionTypes::YouthLeague           => "Youth League",
            CompetitionTypes::DomesticCup           => "Domestic Cup",
            CompetitionTypes::DomesticSuperCup      => "Domestic Super Cup",
            CompetitionTypes::PlayOffs              => "Play-Offs",
            CompetitionTypes::LeagueCup             => "League Cup",
            CompetitionTypes::DomesticYouthCup      => "Domestic Youth Cup",
            CompetitionTypes::ReserveLeague         => "Reserve League",
            CompetitionTypes::FurtherCup            => "Further Cup",
            CompetitionTypes::NationalYouthSuperCup => "National Youth Super Cup",
            CompetitionTypes::InternationalCups     => "International Cups",
            CompetitionTypes::Cups                  => "Cups",
        };
        write!(f, "{}", s)
    }
}