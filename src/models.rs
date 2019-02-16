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
    #[serde(with = "Wrapper", default)]
    pub stages: Option<Vec<Stage>>,
    #[serde(with = "Wrapper", default)]
    pub rounds: Option<Vec<Round>>,
    #[serde(with = "Wrapper", default)]
    pub upcoming: Option<Vec<Fixture>>,
    #[serde(with = "Wrapper", default)]
    pub results: Option<Vec<Fixture>>,
    #[serde(with = "Wrapper", default)]
    pub groups: Option<Vec<Group>>,
    #[serde(with = "Wrapper", default)]
    pub goalscorers: Option<Vec<Goals>>,
    #[serde(with = "Wrapper", default)]
    pub assistscorers: Option<Vec<Assists>>,
    #[serde(with = "Wrapper", default)]
    pub cardscorers: Option<Vec<Cards>>,
    #[serde(with = "Wrapper", default, rename = "aggregatedGoalscorers")]
    pub aggregated_goalscorers: Option<Vec<Goals>>,
    #[serde(with = "Wrapper", default, rename = "aggregatedAssistscorers")]
    pub aggregated_assistscorers: Option<Vec<Assists>>,
    #[serde(with = "Wrapper", default, rename = "aggregatedCardscorers")]
    pub aggregated_cardscorers: Option<Vec<Cards>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Fixture {
    pub id: i64,
    pub league_id: i64,
    pub season_id: i64,
    pub stage_id: i64,
    pub round_id: Option<i64>,
    pub group_id: Option<i64>,
    pub aggregate_id: Option<i64>,
    pub venue_id: Option<i64>,
    pub referee_id: Option<i64>,
    pub localteam_id: i64,
    pub visitorteam_id: i64,
    pub weather_report: Option<WeatherReport>,
    pub commentaries: Option<bool>,
    pub attendance: Option<i64>,
    pub pitch: Option<String>,
    pub winning_odds_calculated: bool,
    pub formations: Formations,
    pub scores: Scores,
    pub time: Time,
    pub coaches: Coaches,
    pub standings: Standings,
    pub assistants: Assistants,
    pub leg: String,
    pub colors: Option<Colors>,
    pub deleted: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WeatherReport {
    pub code: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub icon: String,
    pub temperature: Temperature,
    pub clouds: String,
    pub humidity: String,
    pub pressure: Option<i64>,
    pub wind: Wind,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Temperature {
    pub temp: f64,
    pub unit: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Wind {
    pub speed: String,
    pub degree: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Formations {
    pub localteam_formation: Option<String>,
    pub visitorteam_formation: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Scores {
    pub localteam_score: i64,
    pub visitorteam_score: i64,
    pub localteam_pen_score: Option<i64>,
    pub visitorteam_pen_score: Option<i64>,
    pub ht_score: Option<String>,
    pub ft_score: Option<String>,
    pub et_score: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Time {
    pub status: String,
    pub starting_at: StartingAt,
    pub minute: Option<i64>,
    pub second: Option<i64>,
    pub added_time: Option<i64>,
    pub extra_minute: Option<i64>,
    pub injury_time: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StartingAt {
    pub date_time: String,
    pub date: String,
    pub time: String,
    pub timestamp: i64,
    pub timezone: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Coaches {
    pub localteam_coach_id: Option<i64>,
    pub visitorteam_coach_id: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Standings {
    pub localteam_position: Option<i64>,
    pub visitorteam_position: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Assistants {
    pub first_assistant_id: Option<i64>,
    pub second_assistant_id: Option<i64>,
    pub fourth_official_id: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Colors {
    pub localteam: ColorKit,
    pub visitorteam: ColorKit,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ColorKit {
    pub color: Option<String>,
    pub kit_colors: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Goals {
    pub position: i64,
    pub season_id: i64,
    pub player_id: i64,
    pub team_id: i64,
    pub stage_id: Option<i64>,
    pub goals: i64,
    pub penalty_goals: i64,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Assists {
    pub position: i64,
    pub season_id: i64,
    pub player_id: i64,
    pub team_id: i64,
    pub stage_id: Option<i64>,
    pub assists: i64,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Cards {
    pub position: i64,
    pub season_id: i64,
    pub player_id: i64,
    pub team_id: i64,
    pub stage_id: Option<i64>,
    pub yellowcards: i64,
    pub redcards: i64,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Round {
    pub id: i64,
    pub name: i64,
    pub league_id: i64,
    pub season_id: i64,
    pub stage_id: i64,
    pub start: String,
    pub end: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Stage {
    pub id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub league_id: i64,
    pub season_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub league_id: i64,
    pub season_id: i64,
    pub round_id: Option<i64>,
    pub round_name: Option<i64>,
    pub stage_id: i64,
    pub stage_name: String,
}



