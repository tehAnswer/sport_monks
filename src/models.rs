// use super::wrapper::Wrapper;
use serde::{Deserialize, Deserializer};

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Wrapper<T> {
    pub data: T,
}

impl<T> Wrapper<T> {
    fn deserialize<'de, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        let wrapper = <Self as Deserialize>::deserialize(deserializer)?;
        Ok(wrapper.data)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Continent {
    pub id: u64,
    pub name: String,
    #[serde(with = "Wrapper", default)]
    pub countries: Option<Vec<Country>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Country {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub extra: Option<CountryExtra>,
    #[serde(with = "Wrapper", default)]
    pub leagues: Option<Vec<League>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CountryExtra {
    pub continent: Option<String>,
    pub sub_region: Option<String>,
    pub world_region: Option<String>,
    pub fifa: Option<String>,
    pub iso: Option<String>,
    pub longitude: Option<String>,
    pub latitude: Option<String>,
    pub flag: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct League {
    pub id: i64,
    pub legacy_id: i64,
    pub country_id: i64,
    pub logo_path: String,
    pub name: String,
    pub is_cup: bool,
    pub current_season_id: i64,
    pub current_round_id: Option<i64>,
    pub current_stage_id: i64,
    pub live_standings: bool,
    pub coverage: Coverage,
    #[serde(with = "Wrapper", default)]
    pub seasons: Option<Vec<Season>>,
    #[serde(with = "Wrapper", default)]
    pub season: Option<Season>,
    #[serde(with = "Wrapper", default)]
    pub country: Option<Country>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Coverage {
    pub topscorer_goals: bool,
    pub topscorer_assists: bool,
    pub topscorer_cards: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Season {
    pub id: i64,
    pub name: String,
    pub league_id: i64,
    pub is_current_season: bool,
    pub current_round_id: Option<i64>,
    pub current_stage_id: Option<i64>,
}

