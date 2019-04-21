use super::super::errors::SportMonksError;
use super::super::models::{Fixture,Wrapper};
use super::super::gateway::{Gateway,Options};
use chrono::{Date, Utc};



pub struct FixtureGateway {
    gateway: Gateway
}

impl FixtureGateway {
    pub fn new(gateway: Gateway) -> FixtureGateway {
        FixtureGateway { gateway }
    }
    pub fn find(&self, id: i64) -> Result<Wrapper<Fixture>, SportMonksError> {
        let path = format!("/fixtures/{}", id);
        self.gateway.get(&path, Options::empty())
    }
    
    pub fn find_with(&self, id: i64, options: Options) -> Result<Wrapper<Fixture>, SportMonksError> {
        let path = format!("/fixtures/{}", id);
        self.gateway.get(&path, options)
    }

    pub fn on(&self, date: Date<Utc>) -> Result<Wrapper<Vec<Fixture>>, SportMonksError> {
        let path = format!("/fixtures/date/{}", date.format("%Y-%m-%d"));
        self.gateway.get(&path, Options::empty())
    }

    pub fn on_with(&self, date: Date<Utc>, options: Options) -> Result<Wrapper<Vec<Fixture>>, SportMonksError> {
        let path = format!("/fixtures/date/{}", date.format("%Y-%m-%d"));
        self.gateway.get(&path, options)
    }
    
    pub fn between(&self, start: Date<Utc>, end: Date<Utc>) -> Result<Wrapper<Vec<Fixture>>, SportMonksError> {
        let path = format!("/fixtures/between/{}/{}", start.format("%Y-%m-%d"), end.format("%Y-%m-%d"));
        self.gateway.get(&path, Options::empty())
    }   
    
    pub fn between_with(&self, start: Date<Utc>, end: Date<Utc>, options: Options) -> Result<Wrapper<Vec<Fixture>>, SportMonksError> {
        let path = format!("/fixtures/between/{}/{}", start.format("%Y-%m-%d"), end.format("%Y-%m-%d"));
        self.gateway.get(&path, options)
    }
    
    pub fn team_between(&self, team_id: i64, start: Date<Utc>, end: Date<Utc>) -> Result<Wrapper<Vec<Fixture>>, SportMonksError> {
        let path = format!("/fixtures/between/{}/{}/{}", start.format("%Y-%m-%d"), end.format("%Y-%m-%d"), team_id);
        self.gateway.get(&path, Options::empty())
    }

    pub fn team_between_with(&self, team_id: i64, start: Date<Utc>, end: Date<Utc>, options: Options) -> Result<Wrapper<Vec<Fixture>>, SportMonksError> {
        let path = format!("/fixtures/between/{}/{}/{}", start.format("%Y-%m-%d"), end.format("%Y-%m-%d"), team_id);
        self.gateway.get(&path, options)
    }
    
    pub fn filter(&self, fixture_ids: &[i64]) -> Result<Wrapper<Vec<Fixture>>, SportMonksError> {
        self.filter_with(fixture_ids, Options::empty())
    }
    
    pub fn filter_with(&self, fixture_ids: &[i64], options: Options) -> Result<Wrapper<Vec<Fixture>>, SportMonksError> {
        let list_of_ids: Vec<String> = fixture_ids.iter().map(ToString::to_string).collect();
        let path = format!("/fixtures/multi/{}", list_of_ids.join(","));
        self.gateway.get(&path, options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;
    use std::fs;
    use std::path::Path;
    use chrono::TimeZone;

    #[test]
    fn it_finds_fixtures_by_id() {
        let body = fs::read_to_string(Path::new("src/support/fixtures/find.json")).expect("Fixtures:");
        let m = mock("GET", "/fixtures/11414776?api_token=1234")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = FixtureGateway::new(Gateway::new("1234".into()));
        let fixture = instance.find(11414776).unwrap().data;

        m.assert();

        assert_eq!(fixture.id, 11414776);
        assert_eq!(fixture.league_id, 2);
        assert_eq!(fixture.season_id, 12950);
        assert_eq!(fixture.round_id, None);
        assert_eq!(fixture.group_id, None);
        assert_eq!(fixture.aggregate_id, Some(17561));
        assert_eq!(fixture.venue_id, Some(2085));
        assert_eq!(fixture.referee_id, None);
        assert_eq!(fixture.localteam_id, 67);
        assert_eq!(fixture.visitorteam_id, 9);
        assert_eq!(fixture.weather_report, None);
        assert_eq!(fixture.commentaries, Some(false));
        assert_eq!(fixture.attendance, None);
        assert_eq!(fixture.pitch, None);
        assert_eq!(fixture.winning_odds_calculated, false);
        
        assert_eq!(fixture.formations.localteam_formation, None);
        assert_eq!(fixture.formations.visitorteam_formation, None);

        assert_eq!(fixture.scores.localteam_score, 0);
        assert_eq!(fixture.scores.visitorteam_score, 0);
        assert_eq!(fixture.scores.localteam_pen_score, None);
        assert_eq!(fixture.scores.visitorteam_pen_score, None);
        assert_eq!(fixture.scores.ht_score, None);
        assert_eq!(fixture.scores.ft_score, None);
        assert_eq!(fixture.scores.et_score, None);

        assert_eq!(&fixture.time.status, "NS");
        assert_eq!(fixture.time.starting_at.date_time, "2019-02-20 20:00:00");
        assert_eq!(fixture.time.starting_at.date, "2019-02-20");
        assert_eq!(fixture.time.starting_at.time, "20:00:00");
        assert_eq!(fixture.time.starting_at.timestamp, 1550692800);
        assert_eq!(fixture.time.starting_at.timezone, "UTC");

        assert_eq!(fixture.time.minute, None);
        assert_eq!(fixture.time.second, None);
        assert_eq!(fixture.time.added_time, None);
        assert_eq!(fixture.time.extra_minute, None);
        assert_eq!(fixture.time.injury_time, None);

        assert_eq!(fixture.coaches.localteam_coach_id, Some(893655));
        assert_eq!(fixture.coaches.visitorteam_coach_id, Some(455361));

        assert_eq!(fixture.standings.localteam_position, None);
        assert_eq!(fixture.standings.visitorteam_position, None);
        
        assert_eq!(fixture.assistants.first_assistant_id, None);
        assert_eq!(fixture.assistants.second_assistant_id, None);
        assert_eq!(fixture.assistants.fourth_official_id, None);

        assert_eq!(fixture.leg, Some("1/2".to_string()));
        assert_eq!(fixture.colors, None);
        assert_eq!(fixture.deleted, false);

    }

    #[test]
    fn it_finds_fixtures_by_id_with_details() {
        let body = fs::read_to_string(Path::new("src/support/fixtures/find_with.json")).expect("Fixtures:");
        let m = mock("GET", "/fixtures/11414776?api_token=1234&include=visitorTeam%2ClocalTeam")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = FixtureGateway::new(Gateway::new("1234".into()));
        let opts = Options::builder().include(&vec!["visitorTeam", "localTeam"]);
        let fixture = instance.find_with(11414776, opts).unwrap().data;

        m.assert();

        assert!(fixture.local_team.is_some());
        assert!(fixture.visitor_team.is_some());

        let local_team = fixture.local_team.clone().unwrap();
        assert_eq!(local_team.id, 67);
        assert_eq!(local_team.legacy_id, Some(42));
        assert_eq!(&local_team.name, "Schalke 04");
        assert_eq!(local_team.short_code, Some("S04".into()));
        assert_eq!(local_team.country_id, 11);
        assert_eq!(local_team.twitter, Some("@s04".to_string()));
        assert_eq!(local_team.national_team, false);
        assert_eq!(local_team.founded, Some(1904));
        assert_eq!(local_team.logo_path, Some("https://cdn.sportmonks.com/images/soccer/teams/3/67.png".into()));
        assert_eq!(local_team.venue_id, Some(2138));
        assert_eq!(local_team.current_season_id, None);

        let visitor_team = fixture.visitor_team.clone().unwrap();
        assert_eq!(visitor_team.id, 9);
        assert_eq!(visitor_team.legacy_id, Some(127));
        assert_eq!(&visitor_team.name, "Manchester City");
        assert_eq!(visitor_team.short_code, Some("MCI".into()));
        assert_eq!(visitor_team.twitter, Some("@ManCity".to_string()));
        assert_eq!(visitor_team.country_id, 462);
        assert_eq!(visitor_team.national_team, false);
        assert_eq!(visitor_team.founded, Some(1880));
        assert_eq!(visitor_team.logo_path, Some("https://cdn.sportmonks.com/images/soccer/teams/9/9.png".into()));
        assert_eq!(visitor_team.venue_id, Some(151));
        assert_eq!(visitor_team.current_season_id, None);
    }


    #[test]
    fn it_finds_fixtures_happening_on_a_day_with_details() {
        let body = fs::read_to_string(Path::new("src/support/fixtures/on_with.json")).expect("Fixtures:");
        let m = mock("GET", "/fixtures/date/2019-02-16?api_token=1234&include=substitutions%2Cgoals%2Ccards%2Cevents")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = FixtureGateway::new(Gateway::new("1234".into()));
        let includes = vec!["substitutions,goals,cards,events"];
        let opts = Options::builder().include(&includes);
        let result = instance.on_with(Utc.ymd(2019, 2, 16), opts).unwrap();
        m.assert();

        let substitutions = result.data[0].substitutions.clone().unwrap();
        assert_eq!(substitutions.len(), 4);
        assert_eq!(substitutions[0].id, 1685506004);
        assert_eq!(&substitutions[0].team_id, "2379");
        assert_eq!(&substitutions[0].kind, "subst");
        assert_eq!(substitutions[0].fixture_id, 1685506);
        assert_eq!(substitutions[0].player_in_id, 12668);
        assert_eq!(&substitutions[0].player_in_name, "J. Janse");
        assert_eq!(substitutions[0].player_out_id, 22954);
        assert_eq!(&substitutions[0].player_out_name, "R. Castelen");
        assert_eq!(substitutions[0].minute, 55);
        assert_eq!(substitutions[0].extra_minute, None);
        assert_eq!(substitutions[0].injuried, None);

        let goals = result.data[0].goals.clone().unwrap();
        assert_eq!(goals.len(), 2);
        assert_eq!(goals[0].id, 1685506001);
        assert_eq!(&goals[0].team_id, "2379");
        assert_eq!(&goals[0].kind, "goal");
        assert_eq!(goals[0].fixture_id, 1685506);
        assert_eq!(goals[0].player_id, 28212);
        assert_eq!(&goals[0].player_name, "V. van Crooij");
        assert_eq!(goals[0].player_assist_id, Some(31052));
        assert_eq!(goals[0].player_assist_name, Some("L. Thy".to_string()));
        assert_eq!(goals[0].minute, 2);
        assert_eq!(goals[0].extra_minute, None);
        assert_eq!(goals[0].reason, None);
        assert_eq!(&goals[0].result, "1-0");

        let cards = result.data[0].cards.clone().unwrap();
        assert_eq!(cards.len(), 4);
        assert_eq!(cards[0].id, 1685506003);
        assert_eq!(&cards[0].team_id, "2345");
        assert_eq!(&cards[0].kind, "yellowcard");
        assert_eq!(cards[0].fixture_id, 1685506);
        assert_eq!(cards[0].player_id, 24430);
        assert_eq!(&cards[0].player_name, "M. te Wierik");
        assert_eq!(cards[0].minute, 20);
        assert_eq!(cards[0].extra_minute, None);
        assert_eq!(cards[0].reason, None);

        let events = result.data[0].events.clone().unwrap();
        assert_eq!(events[0].id, 1685506001);
        assert_eq!(&events[0].team_id, "2379");
        assert_eq!(&events[0].kind, "goal");
        assert_eq!(events[0].fixture_id, 1685506);
        assert_eq!(events[0].player_id, 28212);
        assert_eq!(&events[0].player_name, "V. van Crooij");
        assert_eq!(events[0].related_player_id, Some(31052));
        assert_eq!(events[0].related_player_name, Some("L. Thy".to_string()));
        assert_eq!(events[0].minute, 2);
        assert_eq!(events[0].extra_minute, None);
        assert_eq!(events[0].reason, None);
        assert_eq!(events[0].result, Some("1-0".to_string()));
    }
    
    #[test]
    fn it_finds_fixtures_happening_between_two_dates() {
        let body = fs::read_to_string(Path::new("src/support/fixtures/between_with.json")).expect("Fixtures:");
        let m = mock("GET", "/fixtures/between/2019-02-16/2019-02-18?api_token=1234&include=corners%2Clineup%2Cbench%2Csidelined%2Ccomments%2Ctvstations")
          .with_status(200)
          .with_body(body)
          .create();

        let instance = FixtureGateway::new(Gateway::new("1234".into()));
        let includes = vec!["corners,lineup,bench,sidelined,comments,tvstations"];
        let opts = Options::builder().include(&includes);
        let result = instance.between_with(Utc.ymd(2019, 2, 16), Utc.ymd(2019, 2, 18), opts).unwrap();
        m.assert();

        let corners = result.data[0].corners.clone().unwrap();
        assert_eq!(corners[0].id, 89408);
        assert_eq!(corners[0].team_id, 2345);
        assert_eq!(corners[0].fixture_id, 1685506);
        assert_eq!(corners[0].minute, 9);
        assert_eq!(corners[0].extra_minute, None);
        assert_eq!(corners[0].comment, "1st Corner");

        let lineup = result.data[0].lineup.clone().unwrap();
        assert_eq!(lineup.len(), 22);
        assert_eq!(lineup[0].team_id, 2379);
        assert_eq!(lineup[0].fixture_id, 1685506);
        assert_eq!(lineup[0].player_id, Some(31161));
        assert_eq!(lineup[0].player_name, "Lars Unnerstall");
        assert_eq!(lineup[0].number, Some(1));
        assert_eq!(lineup[0].position, Some("G".to_string()));
        assert_eq!(lineup[0].additional_position, None);
        assert_eq!(lineup[0].formation_position, Some(1));
        assert_eq!(lineup[0].captain, false);

        assert_eq!(lineup[0].stats.shots.shots_on_goal, 0);
        assert_eq!(lineup[0].stats.shots.shots_total, 0);

        assert_eq!(lineup[0].stats.goals.scored, 0);
        assert_eq!(lineup[0].stats.goals.assists, 0);
        assert_eq!(lineup[0].stats.goals.conceded, 1);

        assert_eq!(lineup[0].stats.fouls.drawn, 0);
        assert_eq!(lineup[0].stats.fouls.committed, 0);

        assert_eq!(lineup[0].stats.cards.yellowcards, 0);
        assert_eq!(lineup[0].stats.cards.redcards, 0);

        assert_eq!(lineup[0].stats.passing.total_crosses, 0);
        assert_eq!(lineup[0].stats.passing.crosses_accuracy, 0);
        assert_eq!(lineup[0].stats.passing.passes, 14);
        assert_eq!(lineup[0].stats.passing.passes_accuracy, 50);
        assert_eq!(lineup[0].stats.passing.key_passes, 0);

        assert_eq!(lineup[0].stats.dribbles.attempts, 0);
        assert_eq!(lineup[0].stats.dribbles.success, 0);
        assert_eq!(lineup[0].stats.dribbles.dribbled_past, 0);

        assert_eq!(lineup[0].stats.duels.total, 0);
        assert_eq!(lineup[0].stats.duels.won, 0);

        assert_eq!(lineup[0].stats.other.offsides, 0);
        assert_eq!(lineup[0].stats.other.saves, 4);
        assert_eq!(lineup[0].stats.other.inside_box_saves, 0);
        assert_eq!(lineup[0].stats.other.pen_scored, 0);
        assert_eq!(lineup[0].stats.other.pen_missed, 0);
        assert_eq!(lineup[0].stats.other.pen_saved, 0);
        assert_eq!(lineup[0].stats.other.pen_committed, 0);
        assert_eq!(lineup[0].stats.other.pen_won, 0);
        assert_eq!(lineup[0].stats.other.hit_woodwork, 0);
        assert_eq!(lineup[0].stats.other.tackles, 0);
        assert_eq!(lineup[0].stats.other.blocks, 0);
        assert_eq!(lineup[0].stats.other.interceptions, 0);
        assert_eq!(lineup[0].stats.other.clearances, 1);
        assert_eq!(lineup[0].stats.other.dispossesed, 0);
        assert_eq!(lineup[0].stats.other.minutes_played, 90);

        let bench = result.data[0].bench.clone().unwrap();
        assert_eq!(bench[0].team_id, 2379);
        assert_eq!(bench[0].fixture_id, 1685506);
        assert_eq!(bench[0].player_id, Some(2846850));
        assert_eq!(bench[0].player_name, "Bram Verbong");
        assert_eq!(bench[0].number, Some(31));
        assert_eq!(bench[0].position, Some("G".to_string()));
        assert_eq!(bench[0].additional_position, None);
        assert_eq!(bench[0].formation_position, None);
        assert_eq!(bench[0].captain, false);

        assert_eq!(bench[0].stats.shots.shots_on_goal, 0);
        assert_eq!(bench[0].stats.shots.shots_total, 0);

        assert_eq!(bench[0].stats.goals.scored, 0);
        assert_eq!(bench[0].stats.goals.assists, 0);
        assert_eq!(bench[0].stats.goals.conceded, 0);

        assert_eq!(bench[0].stats.fouls.drawn, 0);
        assert_eq!(bench[0].stats.fouls.committed, 0);

        assert_eq!(bench[0].stats.cards.yellowcards, 0);
        assert_eq!(bench[0].stats.cards.redcards, 0);

        assert_eq!(bench[0].stats.passing.total_crosses, 0);
        assert_eq!(bench[0].stats.passing.crosses_accuracy, 0);
        assert_eq!(bench[0].stats.passing.passes, 0);
        assert_eq!(bench[0].stats.passing.passes_accuracy, 0);
        assert_eq!(bench[0].stats.passing.key_passes, 0);

        assert_eq!(bench[0].stats.dribbles.attempts, 0);
        assert_eq!(bench[0].stats.dribbles.success, 0);
        assert_eq!(bench[0].stats.dribbles.dribbled_past, 0);

        assert_eq!(bench[0].stats.duels.total, 0);
        assert_eq!(bench[0].stats.duels.won, 0);

        assert_eq!(bench[0].stats.other.offsides, 0);
        assert_eq!(bench[0].stats.other.saves, 0);
        assert_eq!(bench[0].stats.other.inside_box_saves, 0);
        assert_eq!(bench[0].stats.other.pen_scored, 0);
        assert_eq!(bench[0].stats.other.pen_missed, 0);
        assert_eq!(bench[0].stats.other.pen_saved, 0);
        assert_eq!(bench[0].stats.other.pen_committed, 0);
        assert_eq!(bench[0].stats.other.pen_won, 0);
        assert_eq!(bench[0].stats.other.hit_woodwork, 0);
        assert_eq!(bench[0].stats.other.tackles, 0);
        assert_eq!(bench[0].stats.other.blocks, 0);
        assert_eq!(bench[0].stats.other.interceptions, 0);
        assert_eq!(bench[0].stats.other.clearances, 0);
        assert_eq!(bench[0].stats.other.dispossesed, 0);
        assert_eq!(bench[0].stats.other.minutes_played, 0);

        let sidelined = result.data[0].sidelined.clone().unwrap();
        assert_eq!(sidelined[0].team_id, Some(2379));
        assert_eq!(sidelined[0].fixture_id, Some(1685506));
        assert_eq!(sidelined[0].player_id, 24047);
        assert_eq!(sidelined[0].player_name, Some("J. Promes".to_string()));
        assert_eq!(&sidelined[0].reason, "Hamstring");

        let comments = result.data[0].comments.clone().unwrap();
        assert_eq!(comments.len(), 0);

        let tvstations = result.data[0].tvstations.clone().unwrap();
        assert_eq!(tvstations[0].fixture_id, 1685506);
        assert_eq!(&tvstations[0].tvstation, "bet365");
    }

    #[test]
    fn it_finds_fixtures_happening_between_two_dates_for_a_team() {
        let body = fs::read_to_string(Path::new("src/support/fixtures/team_between_with.json")).expect("Fixtures:");
        let m = mock("GET", "/fixtures/between/2019-02-16/2019-02-18/9?api_token=1234&include=highlights%2Cround%2Cstage%2Creferee%2Cvenue%2Codds%2Cinplay%2CflatOdds%2ClocalCoach%2CvisitorCoach%2Ctrends")
          .with_status(200)
          .with_body(body)
          .create();

        let instance = FixtureGateway::new(Gateway::new("1234".into()));
        let includes = vec!["highlights,round,stage,referee,venue,odds,inplay,flatOdds,localCoach,visitorCoach,trends"];
        let opts = Options::builder().include(&includes);
        let result = instance.team_between_with(9, Utc.ymd(2019, 2, 16), Utc.ymd(2019, 2, 18), opts).unwrap();

        m.assert();
        let highlights = result.data[0].highlights.clone().unwrap();
        assert_eq!(highlights[0].fixture_id, 8801067);
        assert_eq!(&highlights[0].location, "https://cc.sporttube.com/embed/gq1CCCG");
        assert_eq!(&highlights[0].created_at.date, "2018-02-13 20:34:44.000000");
        assert_eq!(highlights[0].created_at.timezone_type, 3);
        assert_eq!(&highlights[0].created_at.timezone, "UTC");

        let stage = result.data[0].stage.clone().unwrap();
        assert_eq!(stage.id, 198337);
        assert_eq!(&stage.name, "8th Finals");
        assert_eq!(&stage.kind, "Knock Out");
        assert_eq!(stage.league_id, 2);
        assert_eq!(stage.season_id, 7907);

        let referee = result.data[0].referee.clone().unwrap();
        assert_eq!(referee.id, 21401);
        assert_eq!(&referee.common_name, "J. Eriksson");
        assert_eq!(&referee.fullname, "Jonas Eriksson");
        assert_eq!(&referee.firstname, "Jonas");
        assert_eq!(&referee.lastname, "Eriksson");

        let venue = result.data[0].venue.clone().unwrap();
        assert_eq!(venue.id, 9576);
        assert_eq!(&venue.name, "St. Jakob-Park");
        assert_eq!(&venue.surface, "grass");
        assert_eq!(&venue.address, "Sankt Jakob-Strasse 395");
        assert_eq!(&venue.city, "Basel");
        assert_eq!(venue.capacity, 38512);
        assert_eq!(&venue.image_path, "https://cdn.sportmonks.com/images/soccer/venues/8/9576.png");
        assert_eq!(venue.coordinates, None);

        let odds = result.data[0].odds.clone().unwrap();
        assert_eq!(odds[0].id, 975925);
        assert_eq!(&odds[0].name, "Asian Handicap First Half");

        assert_eq!(odds[0].bookmaker[0].id, 25679219);
        assert_eq!(&odds[0].bookmaker[0].name, "Marathon");

        assert_eq!(&odds[0].bookmaker[0].odds[0].label, "1");
        assert_eq!(&odds[0].bookmaker[0].odds[0].value, "6.00");
        assert_eq!(odds[0].bookmaker[0].odds[0].winning, None);
        assert_eq!(odds[0].bookmaker[0].odds[0].handicap, Some("+0".to_string()));
        assert_eq!(odds[0].bookmaker[0].odds[0].total, None);
        assert_eq!(odds[0].bookmaker[0].odds[0].bookmaker_event_id, None);

        assert_eq!(&odds[0].bookmaker[0].odds[0].last_update.date, "2018-02-13 21:25:28.000000");
        assert_eq!(odds[0].bookmaker[0].odds[0].last_update.timezone_type, 3);
        assert_eq!(&odds[0].bookmaker[0].odds[0].last_update.timezone, "UTC");

        let flat_odds = result.data[0].flat_odds.clone().unwrap();
        assert_eq!(flat_odds[0].bookmaker_id, 25679219);
        assert_eq!(flat_odds[0].bookmaker_event_id, None);
        assert_eq!(flat_odds[0].market_id, 975925);
        assert_eq!(&flat_odds[0].odds[0].label, "1");
        assert_eq!(&flat_odds[0].odds[0].value, "6.00");
        assert_eq!(flat_odds[0].odds[0].winning, None);
        assert_eq!(flat_odds[0].odds[0].handicap, Some("+0".to_string()));
        assert_eq!(flat_odds[0].odds[0].total, None);
        assert_eq!(flat_odds[0].odds[0].bookmaker_event_id, None);

        let local_coach = result.data[0].local_coach.clone().unwrap();
        assert_eq!(local_coach.coach_id, 407770);
        assert_eq!(local_coach.team_id, 468);
        assert_eq!(local_coach.country_id, 62);
        assert_eq!(&local_coach.common_name, "R. Wicky");
        assert_eq!(&local_coach.fullname, "R. Wicky");
        assert_eq!(&local_coach.firstname, "Raphaël");
        assert_eq!(&local_coach.lastname, "Wicky");
        assert_eq!(&local_coach.nationality, "Switzerland");
        assert_eq!(&local_coach.birthdate, "26/04/1977");
        assert_eq!(&local_coach.birthcountry, "Switzerland");
        assert_eq!(&local_coach.birthplace, "Leuggern");
        assert_eq!(&local_coach.image_path, "https://cdn.sportmonks.com/images/soccer/players/26/407770.png");

        let visitor_coach = result.data[0].visitor_coach.clone().unwrap();
        assert_eq!(visitor_coach.coach_id, 455361);
        assert_eq!(visitor_coach.team_id, 9);
        assert_eq!(visitor_coach.country_id, 32);
        assert_eq!(&visitor_coach.common_name, "Guardiola");
        assert_eq!(&visitor_coach.fullname, "Guardiola");
        assert_eq!(&visitor_coach.firstname, "Josep");
        assert_eq!(&visitor_coach.lastname, "Guardiola i Sala");
        assert_eq!(&visitor_coach.nationality, "Spain");
        assert_eq!(&visitor_coach.birthdate, "18/01/1971");
        assert_eq!(&visitor_coach.birthcountry, "Spain");
        assert_eq!(&visitor_coach.birthplace, "Santpedor");
        assert_eq!(&visitor_coach.image_path, "https://cdn.sportmonks.com/images/soccer/players/1/455361.png");

        let trends = result.data[0].trends.clone().unwrap();
        assert_eq!(trends[0].id, 751206);
        assert_eq!(trends[0].fixture_id, 60805);
        assert_eq!(trends[0].team_id, 468);
        assert_eq!(&trends[0].kind, "possession");
        assert_eq!(&trends[0].analyses[1].minute, "1");
        assert_eq!(&trends[0].analyses[1].amount, "23");

        assert_eq!(trends[1].id, 751209);
        assert_eq!(trends[1].fixture_id, 60805);
        assert_eq!(trends[1].team_id, 09);
        assert_eq!(&trends[1].kind, "possession");
        assert_eq!(&trends[1].analyses[1].minute, "1");
        assert_eq!(&trends[1].analyses[1].amount, "77");
    }

    #[test]
    fn it_finds_multiple_fixtures() {
        let body = fs::read_to_string(Path::new("src/support/fixtures/filter_with.json")).expect("Fixtures:");
        let m = mock("GET", "/fixtures/multi/11414776?api_token=1234&include=firstAssistant%2CsecondAssistant%2CfourthOfficial%2Cstats%2Cshootout")
          .with_status(200)
          .with_body(body)
          .create();

        let instance = FixtureGateway::new(Gateway::new("1234".into()));
        let includes = vec!["firstAssistant,secondAssistant,fourthOfficial,stats,shootout"];
        let opts = Options::builder().include(&includes);
        let result = instance.filter_with(&vec![11414776], opts).unwrap();

        m.assert();

        let first_assistant = result.data[0].first_assistant.clone().unwrap(); 
        assert_eq!(first_assistant.id, 12734);
        assert_eq!(&first_assistant.common_name, "E. Zeinstra");
        assert_eq!(&first_assistant.fullname, "Erwin Zeinstra");
        assert_eq!(&first_assistant.firstname, "Erwin");
        assert_eq!(&first_assistant.lastname, "Zeinstra");
    
        let second_assistant = result.data[0].second_assistant.clone().unwrap(); 
        assert_eq!(second_assistant.id, 11645);
        assert_eq!(&second_assistant.common_name, "J. Hubers");
        assert_eq!(&second_assistant.fullname, "Johan Hubers");
        assert_eq!(&second_assistant.firstname, "Johan");
        assert_eq!(&second_assistant.lastname, "Hubers");
        
        let fourth_official = result.data[0].fourth_official.clone().unwrap(); 
        assert_eq!(fourth_official.id, 14882,);
        assert_eq!(&fourth_official.common_name, "F. Van Herk");
        assert_eq!(&fourth_official.fullname, "Freek Van Herk");
        assert_eq!(&fourth_official.firstname, "Freek");
        assert_eq!(&fourth_official.lastname, "Van Herk");
        
        let stats = result.data[0].stats.clone().unwrap();
        assert_eq!(stats[0].team_id, 2379);
        assert_eq!(stats[0].fixture_id, 1685506);

        assert_eq!(stats[0].shots.total, 13);
        assert_eq!(stats[0].shots.ongoal, 4);
        assert_eq!(stats[0].shots.offgoal, 7);
        assert_eq!(stats[0].shots.blocked, 2);
        assert_eq!(stats[0].shots.insidebox, 7);
        assert_eq!(stats[0].shots.outsidebox, 6);

        assert_eq!(stats[0].passes.total, 467);
        assert_eq!(stats[0].passes.accurate, 360);
        assert_eq!(stats[0].passes.percentage, 77);

        assert_eq!(stats[0].attacks.attacks, 123);
        assert_eq!(stats[0].attacks.dangerous_attacks, 63);

        assert_eq!(stats[0].fouls, 11);
        assert_eq!(stats[0].corners, 4);
        assert_eq!(stats[0].offsides, 1);
        assert_eq!(stats[0].possessiontime, 46);
        assert_eq!(stats[0].yellowcards, 1);
        assert_eq!(stats[0].redcards, 0);
        assert_eq!(stats[0].saves, 4);
        assert_eq!(stats[0].substitutions, 2);
        assert_eq!(stats[0].goal_kick, 0);
        assert_eq!(stats[0].goal_attempts, 0);
        assert_eq!(stats[0].free_kick, 0);
        assert_eq!(stats[0].throw_in, 0);
        assert_eq!(stats[0].ball_safe, 0);

        assert_eq!(result.data[0].possesion_won_by_locals(), false);
        assert_eq!(result.data[0].possesion_won_by_visitors(), true);
        assert_eq!(result.data[0].locals_won(), false);
        assert_eq!(result.data[0].visitors_won(), false);
    }

    #[test]
    fn it_finds_multiple_stuff_regression() {
        let body = fs::read_to_string(Path::new("src/support/fixtures/find_with_many_stuff.json")).expect("Fixtures:");
        let m = mock("GET", "/fixtures/11414776?api_token=1234&include=goals%2Ccards%2Cevents%2Clineup%2Cbench%2Codds%2Cstats")
          .with_status(200)
          .with_body(body)
          .create();
        let instance = FixtureGateway::new(Gateway::new("1234".into()));
        let includes = vec!["goals,cards,events,lineup,bench,odds,stats"];
        let opts = Options::builder().include(&includes);
        let result = instance.find_with(11414776, opts);

        m.assert();
        assert!(result.is_ok());
    }

   #[test]
    fn it_finds_games_between_two_dates_with_details_regression() {
        let body = fs::read_to_string(Path::new("src/support/fixtures/fixtures_between_with_stats.json")).expect("Fixtures:");
        let m = mock("GET", "/fixtures/between/2018-08-01/2019-04-21?api_token=1234&include=goals%2Clineup%2Cbench%2Cstats&leagues=2%2C5%2C8%2C72%2C82%2C301%2C384%2C564")
          .with_status(200)
          .with_body(body)
          .create();

        let instance = FixtureGateway::new(Gateway::new("1234".into()));
        let includes = vec!["goals,lineup,bench,stats"];
        let leagues = "2,5,8,72,82,301,384,564";
        let opts = Options::builder().include(&includes).param("leagues", &leagues);
        let result = instance.between_with(Utc.ymd(2018, 8, 1), Utc.ymd(2019, 4, 21), opts);

        println!("{:?}", result);
        m.assert();
        assert!(result.is_ok());
    }
}
