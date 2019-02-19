use serde::{Deserialize, Deserializer};
use serde::de::{DeserializeOwned};

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
    pub leg: Option<String>,
    pub colors: Option<Colors>,
    pub deleted: bool,
    #[serde(with = "Wrapper", default, rename = "visitorTeam")]
    pub visitor_team: Option<Team>,
    #[serde(with = "Wrapper", default, rename = "localTeam")]
    pub local_team: Option<Team>,
    #[serde(with = "Wrapper", default)]
    pub substitutions: Option<Vec<Substitution>>,
    #[serde(with = "Wrapper", default)]
    pub goals: Option<Vec<GoalEvent>>,
    #[serde(with = "Wrapper", default)]
    pub cards: Option<Vec<CardEvent>>,
    #[serde(with = "Wrapper", default)]
    pub events: Option<Vec<Event>>,
    #[serde(with = "Wrapper", default)]
    pub stage: Option<Stage>,
    #[serde(with = "Wrapper", default)]
    pub referee: Option<Referee>,
    #[serde(with = "Wrapper", default)]
    pub venue: Option<Venue>,
    #[serde(with = "Wrapper", default)]
    pub odds: Option<Vec<Odds>>,
    #[serde(with = "Wrapper", default)]
    pub inplay: Option<Vec<Odds>>,
    #[serde(with = "Wrapper", default, rename = "flatOdds")]
    pub flat_odds: Option<Vec<FlatOdds>>,
    #[serde(with = "Wrapper", default, rename = "localCoach")]
    pub local_coach: Option<Coach>,
    #[serde(with = "Wrapper", default, rename = "visitorCoach")]
    pub visitor_coach: Option<Coach>,
    #[serde(with = "Wrapper", default)]
    pub trends: Option<Vec<Trend>>,
    #[serde(with = "Wrapper", default, rename = "firstAssistant")]
    pub first_assistant: Option<Assistant>,
    #[serde(with = "Wrapper", default, rename = "secondAssistant")]
    pub second_assistant: Option<Assistant>,
    #[serde(with = "Wrapper", default, rename = "fourthOfficial")]
    pub fourth_official: Option<Assistant>,
    #[serde(with = "Wrapper", default)]
    pub corners: Option<Vec<Corner>>,
    #[serde(with = "Wrapper", default)]
    pub lineup: Option<Vec<PlayerSlot>>,
    #[serde(with = "Wrapper", default)]
    pub bench: Option<Vec<PlayerSlot>>,
    #[serde(with = "Wrapper", default)]
    pub sidelined: Option<Vec<Sidelined>>,
    #[serde(with = "Wrapper", default)]
    pub comments: Option<Vec<Comment>>,
    #[serde(with = "Wrapper", default)]
    pub tvstations: Option<Vec<TvStation>>,
    #[serde(with = "Wrapper", default)]
    pub highlights: Option<Vec<Highlight>>,
    #[serde(with = "Wrapper", default)]
    pub round: Option<Vec<Round>>,
    #[serde(with = "Wrapper", default)]
    pub group: Option<Vec<Group>>,
    #[serde(with = "Wrapper", default)]
    pub stats: Option<Vec<Stat>>,
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Team {
    pub id: i64,
    pub legacy_id: i64,
    pub name: String,
    pub short_code: String,
    pub twitter: Option<String>,
    pub country_id: i64,
    pub national_team: bool,
    pub founded: i64,
    pub logo_path: String,
    pub venue_id: i64,
    pub current_season_id: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Substitution {
    pub id: i64,
    pub team_id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub fixture_id: i64,
    pub player_in_id: i64,
    pub player_in_name: String,
    pub player_out_id: i64,
    pub player_out_name: String,
    pub minute: i64,
    pub extra_minute: Option<i64>,
    pub injuried: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GoalEvent {
    pub id: i64,
    pub team_id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub fixture_id: i64,
    pub player_id: i64,
    pub player_name: String,
    pub player_assist_id: Option<i64>,
    pub player_assist_name: Option<String>,
    pub minute: i64,
    pub extra_minute: Option<i64>,
    pub reason: Option<String>,
    pub result: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CardEvent {
    pub id: i64,
    pub team_id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub fixture_id: i64,
    pub player_id: i64,
    pub player_name: String,
    pub minute: i64,
    pub extra_minute: Option<i64>,
    pub reason: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Event {
    pub id: i64,
    pub team_id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub fixture_id: i64,
    pub player_id: i64,
    pub player_name: String,
    pub related_player_id: Option<i64>,
    pub related_player_name: Option<String>,
    pub minute: i64,
    pub extra_minute: Option<i64>,
    pub reason: Option<String>,
    pub injuried: Option<bool>,
    pub result: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Highlight {
    pub fixture_id: i64,
    pub location: String,
    pub created_at: CreatedAt,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreatedAt {
    pub date: String,
    pub timezone_type: i64,
    pub timezone: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Referee {
    pub id: i64,
    pub common_name: String,
    pub fullname: String,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Venue {
    pub id: i64,
    pub name: String,
    pub surface: String,
    pub address: String,
    pub city: String,
    pub capacity: i64,
    pub image_path: String,
    pub coordinates: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Odds {
    pub id: i64,
    pub name: String,
    #[serde(with = "Wrapper", default)]
    pub bookmaker: Vec<Bookmaker>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Bookmaker {
    pub id: i64,
    pub name: String,
    #[serde(with = "Wrapper", default)]
    pub odds: Vec<BookmakerOdds>,
}


#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct BookmakerOdds {
    pub label: String,
    #[serde(deserialize_with = "to_string")]
    pub value: String,
    pub winning: Option<bool>,
    pub handicap: Option<String>,
    pub total: Option<String>,
    pub bookmaker_event_id: Option<String>,
    pub last_update: UpdatedAt,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct FlatOdds {
    pub bookmaker_id: i64,
    pub bookmaker_event_id: Option<String>,
    pub market_id: i64,
    pub odds: Vec<BookmakerOdds>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Coach {
    pub coach_id: i64,
    pub team_id: i64,
    pub country_id: i64,
    pub common_name: String,
    pub fullname: String,
    pub firstname: String,
    pub lastname: String,
    pub nationality: String,
    pub birthdate: String,
    pub birthcountry: String,
    pub birthplace: String,
    pub image_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Trend {
    pub id: i64,
    pub fixture_id: i64,
    pub team_id: i64,
    #[serde(rename = "type")]
    pub kind: String,
    pub analyses: Vec<Analysis>,
    pub updated_at: UpdatedAt,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Analysis {
    pub minute: String,
    pub amount: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdatedAt {
    pub date: String,
    pub timezone_type: i64,
    pub timezone: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Assistant {
    pub id: i64,
    pub common_name: String,
    pub fullname: String,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Stat {
    pub team_id: i64,
    pub fixture_id: i64,
    pub shots: Shots,
    pub passes: Passes,
    pub attacks: Attacks,
    pub fouls: i64,
    pub corners: i64,
    pub offsides: i64,
    pub possessiontime: i64,
    pub yellowcards: i64,
    pub redcards: i64,
    pub saves: i64,
    pub substitutions: i64,
    pub goal_kick: Option<i64>,
    pub goal_attempts: Option<i64>,
    pub free_kick: Option<i64>,
    pub throw_in: Option<i64>,
    pub ball_safe: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Shots {
    pub total: i64,
    pub ongoal: i64,
    pub offgoal: i64,
    pub blocked: i64,
    pub insidebox: i64,
    pub outsidebox: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Passes {
    pub total: i64,
    pub accurate: i64,
    pub percentage: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Attacks {
    pub attacks: i64,
    pub dangerous_attacks: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Corner {
    pub id: i64,
    pub team_id: i64,
    pub fixture_id: i64,
    pub minute: i64,
    pub extra_minute: Option<i64>,
    pub comment: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PlayerSlot {
    pub team_id: i64,
    pub fixture_id: i64,
    pub player_id: Option<i64>,
    pub player_name: String,
    pub number: i64,
    pub position: Option<String>,
    pub additional_position: Option<String>,
    pub formation_position: Option<i64>,
    pub captain: bool,
    pub stats: Stats,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Stats {
    pub shots: ShotsPerPlayer,
    pub goals: GoalsPerPlayer,
    pub fouls: FoulsPerPlayer,
    pub cards: CardsPerPlayer,
    pub passing: PassingPerPlayer,
    pub dribbles: DribblesPerPlayer,
    pub duels: DuelsPerPlayer,
    pub other: OtherStatsPerPlayer,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ShotsPerPlayer {
    pub shots_total: Option<i64>,
    pub shots_on_goal: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GoalsPerPlayer {
    pub scored: Option<i64>,
    pub assists: Option<i64>,
    pub conceded: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct FoulsPerPlayer {
    pub drawn: Option<i64>,
    pub committed: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CardsPerPlayer {
    pub yellowcards: Option<i64>,
    pub redcards: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PassingPerPlayer {
    pub total_crosses: Option<i64>,
    pub crosses_accuracy: Option<i64>,
    pub passes: Option<i64>,
    pub passes_accuracy: Option<i64>,
    pub key_passes: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DribblesPerPlayer {
    pub attempts: Option<i64>,
    pub success: Option<i64>,
    pub dribbled_past: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DuelsPerPlayer {
    pub total: Option<i64>,
    pub won: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OtherStatsPerPlayer {
    pub assists: Option<i64>,
    pub offsides: Option<i64>,
    pub saves: Option<i64>,
    pub inside_box_saves: Option<i64>,
    pub pen_scored: Option<i64>,
    pub pen_missed: Option<i64>,
    pub pen_saved: Option<i64>,
    pub pen_committed: Option<i64>,
    pub pen_won: Option<i64>,
    pub hit_woodwork: Option<i64>,
    pub tackles: Option<i64>,
    pub blocks: Option<i64>,
    pub interceptions: Option<i64>,
    pub clearances: Option<i64>,
    pub dispossesed: Option<i64>,
    pub minutes_played: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Sidelined {
    pub team_id: i64,
    pub fixture_id: i64,
    pub player_id: i64,
    pub player_name: String,
    pub reason: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Comment { }

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TvStation {
    pub fixture_id: i64,
    pub tvstation: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum FloatOrString { F64(f64), Stringz(String), I64(i64) }

fn to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    match FloatOrString::deserialize(deserializer)? {
        FloatOrString::F64(v) => Ok(v.to_string()),
        FloatOrString::Stringz(v) => Ok(v),
        FloatOrString::I64(v) => Ok(v.to_string())
    }
    // serde_json::from_str(&j).map_err(Error::custom)
}

















