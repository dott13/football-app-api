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
    pub slug: String,
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
