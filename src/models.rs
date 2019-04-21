use serde::{Deserialize, Deserializer};
use std::str::FromStr;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Wrapper<T> {
    pub data: T,
    pub meta: Option<Meta>
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
pub struct Meta {
    pub pagination: Option<Pagination>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Pagination {
    pub total: i64,
    pub count: i64,
    pub per_page: i64,
    pub current_page: i64,
    pub total_pages: i64,
}


#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Continent {
    pub id: i64,
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

impl Fixture {
    pub fn locals_won(&self) -> bool {
        self.scores.localteam_score > self.scores.visitorteam_score
    }

    pub fn visitors_won(&self) -> bool {
        self.scores.visitorteam_score > self.scores.localteam_score 
    }
    
    pub fn possesion_won_by_locals(&self) -> bool {
        match &self.stats {
            Some(stats) => {
                let index = if stats[0].team_id == self.localteam_id { 0 } else { 1 };
                stats[index].possessiontime > 50
            },
            None => false
        }
    }

    pub fn possesion_won_by_visitors(&self) -> bool {
        match &self.stats {
            Some(stats) => {
                let index = if stats[0].team_id == self.visitorteam_id { 0 } else { 1 };
                stats[index].possessiontime > 50
            },
            None => false
        }
    }
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
    #[serde(deserialize_with = "to_i64")]
    pub goals: i64,
    #[serde(deserialize_with = "to_i64")]
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
    #[serde(deserialize_with = "to_i64")]
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
    pub yellowcards: Option<i64>,
    pub redcards: Option<i64>,
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
    pub legacy_id: Option<i64>,
    pub name: String,
    pub short_code: Option<String>,
    pub twitter: Option<String>,
    pub country_id: i64,
    pub national_team: bool,
    pub founded: Option<i64>,
    pub logo_path: Option<String>,
    pub venue_id: Option<i64>,
    pub current_season_id: Option<i64>,
    #[serde(with = "Wrapper", default)]
    pub country: Option<Country>,
    #[serde(with = "Wrapper", default)]
    pub squad: Option<Vec<PlayerInMatch>>,
    #[serde(with = "Wrapper", default)]
    pub coach: Option<Coach>,
    #[serde(with = "Wrapper", default)]
    pub transfers: Option<Vec<Transfer>>,
    #[serde(with = "Wrapper", default)]
    pub sidelined: Option<Vec<SidelinedOnTeam>>,
    #[serde(with = "Wrapper", default)]
    pub stats: Option<Vec<TeamStats>>,
    #[serde(with = "Wrapper", default)]
    pub venue: Option<Venue>,
    #[serde(with = "Wrapper", default)]
    pub uefaranking: Option<UefaRanking>,
    #[serde(with = "Wrapper", default, rename = "visitorFixtures")]
    pub visitor_fixtures: Option<Vec<Fixture>>,
    #[serde(with = "Wrapper", default, rename = "localFixtures")]
    pub local_fixtures: Option<Vec<Fixture>>,
    #[serde(with = "Wrapper", default, rename = "visitorResults")]
    pub visitor_results: Option<Vec<Fixture>>,
    #[serde(with = "Wrapper", default)]
    pub latest: Option<Vec<Fixture>>, 
    #[serde(with = "Wrapper", default)]
    pub upcoming: Option<Vec<Fixture>>,
    #[serde(with = "Wrapper", default)]
    pub goalscorers: Option<Vec<Goals>>,
    #[serde(with = "Wrapper", default)]
    pub cardscorers: Option<Vec<Cards>>,
    #[serde(with = "Wrapper", default)]
    pub assistscorers: Option<Vec<Assists>>,
    #[serde(with = "Wrapper", default, rename = "aggregatedGoalscorers")]
    pub aggregated_goalscorers: Option<Vec<Goals>>,
    #[serde(with = "Wrapper", default, rename = "aggregatedCardscorers")]
    pub aggregated_cardscorers: Option<Vec<Cards>>, 
    #[serde(with = "Wrapper", default, rename = "aggregatedAssistscorers")]
    pub aggregated_assistscorers: Option<Vec<Assists>>,

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
    pub minute: Option<i64>,
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
    #[serde(deserialize_with = "parse_default")]
    pub shots: Shots,
    #[serde(deserialize_with = "parse_default")]
    pub passes: Passes,
    #[serde(deserialize_with = "parse_default")]
    pub attacks: Attacks,
    #[serde(deserialize_with = "to_i64")]
    pub fouls: i64,
    #[serde(deserialize_with = "to_i64")]
    pub corners: i64,
    #[serde(deserialize_with = "to_i64")]
    pub offsides: i64,
    #[serde(deserialize_with = "to_i64")]
    pub possessiontime: i64,
    #[serde(deserialize_with = "to_i64")]
    pub yellowcards: i64,
    #[serde(deserialize_with = "to_i64")]
    pub redcards: i64,
    #[serde(deserialize_with = "to_i64")]
    pub saves: i64,
    #[serde(deserialize_with = "to_i64")]
    pub substitutions: i64,
    #[serde(deserialize_with = "to_i64")]
    pub goal_kick: i64,
    #[serde(deserialize_with = "to_i64")]
    pub goal_attempts: i64,
    #[serde(deserialize_with = "to_i64")]
    pub free_kick: i64,
    #[serde(deserialize_with = "to_i64")]
    pub throw_in: i64,
    #[serde(deserialize_with = "to_i64")]
    pub ball_safe: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Shots {
    #[serde(deserialize_with = "to_i64")]
    pub total: i64,
    #[serde(deserialize_with = "to_i64")]
    pub ongoal: i64,
    #[serde(deserialize_with = "to_i64")]
    pub offgoal: i64,
    #[serde(deserialize_with = "to_i64")]
    pub blocked: i64,
    #[serde(deserialize_with = "to_i64")]
    pub insidebox: i64,
    #[serde(deserialize_with = "to_i64")]
    pub outsidebox: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Passes {
    #[serde(deserialize_with = "to_i64")]
    pub total: i64,
    #[serde(deserialize_with = "to_i64")]
    pub accurate: i64,
    #[serde(deserialize_with = "to_i64")]
    pub percentage: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Attacks {
    #[serde(deserialize_with = "to_i64")]
    pub attacks: i64,
    #[serde(deserialize_with = "to_i64")]
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
    pub number: Option<i64>,
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
    #[serde(deserialize_with = "to_i64")]
    pub shots_total: i64,
    #[serde(deserialize_with = "to_i64")]
    pub shots_on_goal: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GoalsPerPlayer {
    #[serde(deserialize_with = "to_i64")]
    pub scored: i64,
    #[serde(deserialize_with = "to_i64")]
    pub assists: i64,
    #[serde(deserialize_with = "to_i64")]
    pub conceded: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct FoulsPerPlayer {
    #[serde(deserialize_with = "to_i64")]
    pub drawn: i64,
    #[serde(deserialize_with = "to_i64")]
    pub committed: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CardsPerPlayer {
    #[serde(deserialize_with = "to_i64")]
    pub yellowcards: i64,
    #[serde(deserialize_with = "to_i64")]
    pub redcards: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PassingPerPlayer {
    #[serde(deserialize_with = "to_i64")]
    pub total_crosses: i64,
    #[serde(deserialize_with = "to_i64")]
    pub crosses_accuracy: i64,
    #[serde(deserialize_with = "to_i64")]
    pub passes: i64,
    #[serde(deserialize_with = "to_i64")]
    pub passes_accuracy: i64,
    #[serde(deserialize_with = "to_i64")]
    pub key_passes: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DribblesPerPlayer {
    #[serde(deserialize_with = "to_i64")]
    pub attempts: i64,
    #[serde(deserialize_with = "to_i64")]
    pub success: i64,
    #[serde(deserialize_with = "to_i64")]
    pub dribbled_past: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DuelsPerPlayer {
    #[serde(deserialize_with = "to_i64")]
    pub total: i64,
    #[serde(deserialize_with = "to_i64")]
    pub won: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OtherStatsPerPlayer {
    #[serde(deserialize_with = "to_i64")]
    pub offsides: i64,
    #[serde(deserialize_with = "to_i64")]
    pub saves: i64,
    #[serde(deserialize_with = "to_i64")]
    pub inside_box_saves: i64,
    #[serde(deserialize_with = "to_i64")]
    pub pen_scored: i64,
    #[serde(deserialize_with = "to_i64")]
    pub pen_missed: i64,
    #[serde(deserialize_with = "to_i64")]
    pub pen_saved: i64,
    #[serde(deserialize_with = "to_i64")]
    pub pen_committed: i64,
    #[serde(deserialize_with = "to_i64")]
    pub pen_won: i64,
    #[serde(deserialize_with = "to_i64")]
    pub hit_woodwork: i64,
    #[serde(deserialize_with = "to_i64")]
    pub tackles: i64,
    #[serde(deserialize_with = "to_i64")]
    pub blocks: i64,
    #[serde(deserialize_with = "to_i64")]
    pub interceptions: i64,
    #[serde(deserialize_with = "to_i64")]
    pub clearances: i64,
    #[serde(deserialize_with = "to_i64")]
    pub dispossesed: i64,
    #[serde(deserialize_with = "to_i64")]
    pub minutes_played: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Sidelined {
    pub team_id: Option<i64>,
    #[serde(default)]
    pub fixture_id: Option<i64>,
    pub player_id: i64,
    #[serde(default)]
    pub player_name: Option<String>,
    #[serde(alias = "description")]
    pub reason: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SidelinedOnTeam {
    pub player_id: i64,
    pub team_id: i64,
    pub description: String,
    pub start_date: String,
    pub end_date: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Comment {
    pub fixture_id: i64,
    pub important: bool,
    pub order: Option<i64>,
    pub goal: bool,
    pub minute: Option<i64>,
    pub extra_minute: Option<i64>,
    pub comment: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TvStation {
    pub fixture_id: i64,
    pub tvstation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Standing {
    pub name: String,
    pub league_id: i64,
    pub season_id: i64,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub stage_id: i64,
    pub stage_name: String,
    #[serde(with = "Wrapper", default)]
    pub standings: Vec<StandingPosition>
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StandingPosition {
    pub position: i64,
    pub team_id: i64,
    pub team_name: String,
    pub round_id: i64,
    pub round_name: i64,
    pub group_id: Option<i64>,
    pub group_name: Option<String>,
    pub overall: OverallStats,
    pub home: HomeStats,
    pub away: AwayStats,
    pub total: TotalStats,
    pub result: Option<String>,
    pub points: i64,
    pub recent_form: String,
    pub status: String,
    #[serde(with = "Wrapper", default)]
    pub team: Option<Team>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OverallStats {
    pub games_played: i64,
    pub won: i64,
    pub draw: i64,
    pub lost: i64,
    pub goals_scored: i64,
    pub goals_against: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct HomeStats {
    pub games_played: i64,
    pub won: i64,
    pub draw: i64,
    pub lost: i64,
    pub goals_scored: i64,
    pub goals_against: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AwayStats {
    pub games_played: i64,
    pub won: i64,
    pub draw: i64,
    pub lost: i64,
    pub goals_scored: i64,
    pub goals_against: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TotalStats {
    pub goal_difference: i64,
    pub points: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct LiveStanding {
    pub position: i64,
    pub played: i64,
    pub team_id: i64,
    pub team_name: String,
    pub short_code: String,
    pub team_logo: String,
    pub goals: String,
    pub goal_diff: i64,
    pub wins: i64,
    pub lost: i64,
    pub draws: i64,
    pub points: i64,
    pub description: String,
    pub fairplay_points_lose: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PlayerInMatch {
    pub player_id: i64,
    #[serde(default)]
    pub position_id: Option<i64>,
    pub number: Option<i64>,
    #[serde(deserialize_with = "to_i64")]
    pub captain: i64,
    pub injured: bool,
    #[serde(deserialize_with = "to_i64")]
    pub minutes: i64,
    #[serde(deserialize_with = "to_i64")]
    pub appearences: i64,
    #[serde(deserialize_with = "to_i64")]
    pub lineups: i64,
    #[serde(deserialize_with = "to_i64")]
    pub substitute_in: i64,
    #[serde(deserialize_with = "to_i64")]
    pub substitute_out: i64,
    #[serde(deserialize_with = "to_i64")]
    pub substitutes_on_bench: i64,
    #[serde(deserialize_with = "to_i64")]
    pub goals: i64,
    #[serde(deserialize_with = "to_i64")]
    pub assists: i64,
    pub saves: Option<i64>,
    pub inside_box_saves: Option<i64>,
    pub dispossesed: Option<i64>,
    pub interceptions: Option<i64>,
    #[serde(deserialize_with = "to_i64")]
    pub yellowcards: i64,
    #[serde(deserialize_with = "to_i64")]
    pub yellowred: i64,
    #[serde(deserialize_with = "to_i64")]
    pub redcards: i64,
    #[serde(deserialize_with = "to_i64")]
    pub tackles: i64,
    #[serde(deserialize_with = "to_i64")]
    pub blocks: i64,
    #[serde(deserialize_with = "to_i64")]
    pub hit_post: i64,
    pub fouls: FoulsPerPlayer,
    pub crosses: Crosses,
    pub dribbles: DribblesPerPlayer,
    pub duels: Duels,
    pub passes: PassesPerPlayer,
    pub penalties: Penalties,
    #[serde(with = "Wrapper", default)]
    pub player: Option<Player>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Duels {
    #[serde(deserialize_with = "to_i64")]
    pub total: i64,
    #[serde(deserialize_with = "to_i64")]
    pub won: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PassesPerPlayer {
    #[serde(deserialize_with = "to_i64")]
    pub total: i64,
    #[serde(deserialize_with = "to_i64")]
    pub accuracy: i64,
    #[serde(deserialize_with = "to_i64")]
    pub key_passes: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Penalties {
    #[serde(deserialize_with = "to_i64")]
    pub won: i64,
    #[serde(deserialize_with = "to_i64")]
    pub scores: i64,
    #[serde(deserialize_with = "to_i64")]
    pub missed: i64,
    #[serde(deserialize_with = "to_i64")]
    pub committed: i64,
    #[serde(deserialize_with = "to_i64")]
    pub saves: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Crosses {
    #[serde(deserialize_with = "to_i64")]
    pub total: i64,
    #[serde(deserialize_with = "to_i64")]
    pub accurate: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Transfer {
    pub player_id: i64,
    pub from_team_id: i64,
    pub to_team_id: i64,
    pub season_id: Option<i64>,
    pub transfer: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub date: String,
    pub amount: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TeamStats {
    pub team_id: i64,
    pub season_id: i64,
    pub stage_id: Option<i64>,
    pub win: HomeAwayTotalStat,
    pub draw: HomeAwayTotalStat,
    pub lost: HomeAwayTotalStat,
    pub goals_for: HomeAwayTotalStat,
    pub goals_against: HomeAwayTotalStat,
    pub clean_sheet: HomeAwayTotalStat,
    pub scoring_minutes: Vec<ScoringMinute>,
    pub avg_goals_per_game_scored: HomeAwayTotalStat,
    pub avg_goals_per_game_conceded: HomeAwayTotalStat,
    pub avg_first_goal_scored: HomeAwayTotalStat,
    pub avg_first_goal_conceded: HomeAwayTotalStat,
    pub failed_to_score: HomeAwayTotalStat,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct HomeAwayTotalStat {
    #[serde(deserialize_with = "to_f64")]
    pub total: f64,
    #[serde(deserialize_with = "to_f64")]
    pub home: f64,
    #[serde(deserialize_with = "to_f64")]
    pub away: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ScoringMinute {
    pub period: Vec<Period>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Period {
    pub minute: String,
    #[serde(deserialize_with = "to_i64")]
    pub count: i64,
    pub percentage: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UefaRanking {
    pub team_id: i64,
    pub points: Option<i64>,
    pub coeffiecient: Option<i64>,
    pub position: Option<i64>,
    pub position_status: Option<String>,
    pub position_won_or_lost: Option<i64>
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Player {
    pub player_id: i64,
    pub team_id: Option<i64>,
    pub country_id: Option<i64>,
    pub position_id: Option<i64>,
    pub common_name: Option<String>,
    pub fullname: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub nationality: Option<String>,
    pub birthdate: Option<String>,
    pub birthcountry: Option<String>,
    pub birthplace: Option<String>,
    pub height: Option<String>,
    pub weight: Option<String>,
    pub image_path: Option<String>,
    #[serde(with = "Wrapper", default)]
    pub trophies: Option<Vec<Trophy>>,
    #[serde(with = "Wrapper", default)]
    pub transfers: Option<Vec<Transfer>>,
    #[serde(with = "Wrapper", default)]
    pub sidelined: Option<Vec<Sidelined>>,
    #[serde(with = "Wrapper", default)]
    pub position: Option<Position>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Trophy {
    pub player_id: i64,
    pub status: String,
    pub times: i64,
    pub league: String,
    pub league_id: Option<i64>,
    #[serde(with = "Wrapper", default)]
    pub seasons: Vec<Season>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Position {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum FloatOrString { F64(f64), Stringz(String), I64(i64), Nothing }

fn to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    match FloatOrString::deserialize(deserializer)? {
        FloatOrString::F64(v) => Ok(v.to_string()),
        FloatOrString::Stringz(v) => Ok(v),
        FloatOrString::I64(v) => Ok(v.to_string()),
        FloatOrString::Nothing => Ok("".into()),
    }
}

fn to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    match FloatOrString::deserialize(deserializer)? {
        FloatOrString::F64(v) => Ok(v),
        FloatOrString::Stringz(v) => { Ok(f64::from_str(&v.replace("m", "")).unwrap()) },
        FloatOrString::I64(v) => Ok(v as f64),
        FloatOrString::Nothing => Ok(0.0 as f64),
    }
}

fn to_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    match FloatOrString::deserialize(deserializer)? {
        FloatOrString::F64(v) => Ok(v as i64),
        FloatOrString::Stringz(v) => { Ok(i64::from_str(&v.replace("m", "")).unwrap()) },
        FloatOrString::I64(v) => Ok(v),
        FloatOrString::Nothing => Ok(0),
    }
}

fn parse_default<'de, D, R>(d: D) -> Result<R, D::Error>
  where D: Deserializer<'de>, R: Default + Deserialize<'de> {
    Deserialize::deserialize(d)
        .map(|x: Option<_>| {
            x.unwrap_or(R::default())
        })
}