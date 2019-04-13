use super::super::errors::SportMonksError;
use super::super::models::{Season,Wrapper};
use super::super::gateway::{Gateway,Options};


pub struct SeasonGateway {
    gateway: Gateway
}

impl SeasonGateway {
    pub fn new(gateway: Gateway) -> SeasonGateway {
        SeasonGateway { gateway }
    }

    pub fn all(&self) -> Result<Wrapper<Vec<Season>>, SportMonksError> {
        self.gateway.get("/seasons", Options::empty())
    }
    
    pub fn all_with(&self, options: Options) -> Result<Wrapper<Vec<Season>>, SportMonksError> {
        self.gateway.get("/seasons", options)
    }
    
    pub fn find(&self, id: i64) -> Result<Wrapper<Season>, SportMonksError> {
        let path = format!("/seasons/{}", id);
        self.gateway.get(&path, Options::empty())
    }

    pub fn find_with(&self, id: i64, options: Options) -> Result<Wrapper<Season>, SportMonksError> {
        let path = format!("/seasons/{}", id);
        self.gateway.get(&path, options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;
    use std::fs;
    use std::path::Path;

    #[test]
    fn it_returns_all_the_seasons() {
        let body = fs::read_to_string(Path::new("src/support/seasons/all.json")).expect("Fixtures:");
        let m = mock("GET", "/seasons?api_token=1234")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = SeasonGateway::new(Gateway::new("1234".into()));
        let response = instance.all();
        m.assert();

        assert!(response.is_ok());
        let result = response.unwrap();
        assert_eq!(result.data.len(), 50);
        assert_eq!(result.data[0].id, 5307);
        assert_eq!(result.data[0].league_id, 2);
        assert_eq!(&result.data[0].name, "2005/2006");
        assert_eq!(result.data[0].is_current_season, false);
        assert_eq!(result.data[0].current_round_id, None);
        assert_eq!(result.data[0].current_stage_id, None);
        assert_eq!(result.data[0].rounds, None);
        assert_eq!(result.data[0].stages, None);
        assert_eq!(result.data[0].upcoming, None);
        assert_eq!(result.data[0].results, None);
        assert_eq!(result.data[0].groups, None);
        assert_eq!(result.data[0].goalscorers, None);
        assert_eq!(result.data[0].assistscorers, None);
        assert_eq!(result.data[0].cardscorers, None);
        assert_eq!(result.data[0].aggregated_goalscorers, None);
        assert_eq!(result.data[0].aggregated_assistscorers, None);
        assert_eq!(result.data[0].aggregated_cardscorers, None);
    }

    #[test]
    fn it_returns_a_single_season() {
        let body = fs::read_to_string(Path::new("src/support/seasons/find.json")).expect("Fixtures:");
        let m = mock("GET", "/seasons/12950?api_token=1234")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = SeasonGateway::new(Gateway::new("1234".into()));
        let response = instance.find(12950);
        m.assert();

        assert!(response.is_ok());
        let result = response.unwrap();

        assert_eq!(result.data.id, 12950);
        assert_eq!(&result.data.name, "2018/2019");
        assert_eq!(result.data.league_id, 2);
        assert_eq!(result.data.is_current_season, true);
        assert_eq!(result.data.current_round_id, None);
        assert_eq!(result.data.current_stage_id, Some(7743657));
        assert_eq!(result.data.rounds, None);
        assert_eq!(result.data.stages, None);
        assert_eq!(result.data.upcoming, None);
        assert_eq!(result.data.results, None);
        assert_eq!(result.data.groups, None);
        assert_eq!(result.data.goalscorers, None);
        assert_eq!(result.data.assistscorers, None);
        assert_eq!(result.data.cardscorers, None);
        assert_eq!(result.data.aggregated_goalscorers, None);
        assert_eq!(result.data.aggregated_assistscorers, None);
        assert_eq!(result.data.aggregated_cardscorers, None);
    }

    #[test]
    fn it_returns_a_single_season_with_aggregated_goalscorers() {
        let body = fs::read_to_string(Path::new("src/support/seasons/find_with_aggregated_goal_scorers.json")).expect("Fixtures:");
        let m = mock("GET", "/seasons/12950?api_token=1234&include=aggregatedGoalscorers")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = SeasonGateway::new(Gateway::new("1234".into()));
        let response = instance.find_with(12950, Options::builder().include(&vec!["aggregatedGoalscorers"]));
        m.assert();
        
        assert!(response.is_ok());
        
        let result = response.unwrap();
        assert_eq!(result.data.id, 12950);
        assert_eq!(&result.data.name, "2018/2019");
        assert_eq!(result.data.league_id, 2);
        assert_eq!(result.data.is_current_season, true);
        assert_eq!(result.data.current_round_id, None);
        assert_eq!(result.data.current_stage_id, Some(7743657));
        assert_eq!(result.data.rounds, None);
        assert_eq!(result.data.stages, None);
        assert_eq!(result.data.upcoming, None);
        assert_eq!(result.data.results, None);
        assert_eq!(result.data.groups, None);
        assert_eq!(result.data.goalscorers, None);
        assert_eq!(result.data.assistscorers, None);
        assert_eq!(result.data.cardscorers, None);
        assert_eq!(result.data.aggregated_assistscorers, None);
        assert_eq!(result.data.aggregated_cardscorers, None);
        
        let topscorers = result.data.aggregated_goalscorers.clone().unwrap();
        assert_eq!(topscorers[0].position, 1);
        assert_eq!(topscorers[1].position, 2);
        assert_eq!(topscorers[2].position, 3);
        
        assert_eq!(topscorers[0].season_id, 12950);
        assert_eq!(topscorers[1].season_id, 12950);
        assert_eq!(topscorers[2].season_id, 12950);

        assert_eq!(topscorers[0].player_id, 31000);
        assert_eq!(topscorers[1].player_id, 1351);
        assert_eq!(topscorers[2].player_id, 100668);
        
        assert_eq!(topscorers[0].team_id, 503);
        assert_eq!(topscorers[1].team_id, 629);
        assert_eq!(topscorers[2].team_id, 2673);

        assert_eq!(topscorers[0].stage_id, None);
        assert_eq!(topscorers[1].stage_id, None);
        assert_eq!(topscorers[2].stage_id, None);

        assert_eq!(topscorers[0].goals, 8);
        assert_eq!(topscorers[1].goals, 8);
        assert_eq!(topscorers[2].goals, 7);

        assert_eq!(topscorers[0].penalty_goals, 2);
        assert_eq!(topscorers[1].penalty_goals, 2);
        assert_eq!(topscorers[2].penalty_goals, 1);
        
        assert_eq!(&topscorers[0].kind, "aggregated_goals");
        assert_eq!(&topscorers[1].kind, "aggregated_goals");
        assert_eq!(&topscorers[2].kind, "aggregated_goals");
    }

    #[test]
    fn it_returns_a_single_season_with_assistscorers() {
        let body = fs::read_to_string(Path::new("src/support/seasons/find_with_assistscorers.json")).expect("Fixtures:");
        let m = mock("GET", "/seasons/12950?api_token=1234&include=assistscorers")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = SeasonGateway::new(Gateway::new("1234".into()));
        let response = instance.find_with(12950, Options::builder().include(&vec!["assistscorers"]));
        m.assert();
        
        assert!(response.is_ok());
        
        let result = response.unwrap();
        assert_eq!(result.data.id, 12950);
        assert_eq!(&result.data.name, "2018/2019");
        assert_eq!(result.data.league_id, 2);
        assert_eq!(result.data.is_current_season, true);
        assert_eq!(result.data.current_round_id, None);
        assert_eq!(result.data.current_stage_id, Some(7743657));
        assert_eq!(result.data.rounds, None);
        assert_eq!(result.data.stages, None);
        assert_eq!(result.data.upcoming, None);
        assert_eq!(result.data.results, None);
        assert_eq!(result.data.groups, None);
        assert_eq!(result.data.goalscorers, None);
        assert_eq!(result.data.cardscorers, None);
        assert_eq!(result.data.aggregated_assistscorers, None);
        assert_eq!(result.data.aggregated_cardscorers, None);

        let topscorers = result.data.assistscorers.clone().unwrap();
        assert_eq!(topscorers[0].position, 1);
        assert_eq!(topscorers[1].position, 2);
        assert_eq!(topscorers[2].position, 3);
        
        assert_eq!(topscorers[0].season_id, 12950);
        assert_eq!(topscorers[1].season_id, 12950);
        assert_eq!(topscorers[2].season_id, 12950);

        assert_eq!(topscorers[0].player_id, 67827);
        assert_eq!(topscorers[1].player_id, 108293);
        assert_eq!(topscorers[2].player_id, 100668);
        
        assert_eq!(topscorers[0].team_id, 674);
        assert_eq!(topscorers[1].team_id, 649);
        assert_eq!(topscorers[2].team_id, 2673);

        assert_eq!(topscorers[0].stage_id, Some(7664692));
        assert_eq!(topscorers[1].stage_id, Some(7664692));
        assert_eq!(topscorers[2].stage_id, Some(7664692));

        assert_eq!(topscorers[0].assists, Some(2));
        assert_eq!(topscorers[1].assists, Some(2));
        assert_eq!(topscorers[2].assists, Some(1));
        
        assert_eq!(&topscorers[0].kind, "assists");
        assert_eq!(&topscorers[1].kind, "assists");
        assert_eq!(&topscorers[2].kind, "assists");
    }

    #[test]
    fn it_returns_a_single_season_with_cardscorers() {
        let body = fs::read_to_string(Path::new("src/support/seasons/find_with_card_scorers.json")).expect("Fixtures:");
        let m = mock("GET", "/seasons/12950?api_token=1234&include=cardscorers")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = SeasonGateway::new(Gateway::new("1234".into()));
        let response = instance.find_with(12950, Options::builder().include(&vec!["cardscorers"]));
        m.assert();
        
        assert!(response.is_ok());
        
        let result = response.unwrap();
        assert_eq!(result.data.id, 12950);
        assert_eq!(&result.data.name, "2018/2019");
        assert_eq!(result.data.league_id, 2);
        assert_eq!(result.data.is_current_season, true);
        assert_eq!(result.data.current_round_id, None);
        assert_eq!(result.data.current_stage_id, Some(7743657));
        assert_eq!(result.data.rounds, None);
        assert_eq!(result.data.stages, None);
        assert_eq!(result.data.upcoming, None);
        assert_eq!(result.data.results, None);
        assert_eq!(result.data.groups, None);
        assert_eq!(result.data.goalscorers, None);
        assert_eq!(result.data.assistscorers, None);
        assert_eq!(result.data.aggregated_assistscorers, None);
        assert_eq!(result.data.aggregated_cardscorers, None);

        let topscorers = result.data.cardscorers.clone().unwrap();
        assert_eq!(topscorers[0].position, 1);
        assert_eq!(topscorers[1].position, 2);
        assert_eq!(topscorers[2].position, 3);
        
        assert_eq!(topscorers[0].season_id, 12950);
        assert_eq!(topscorers[1].season_id, 12950);
        assert_eq!(topscorers[2].season_id, 12950);

        assert_eq!(topscorers[0].player_id, 74038);
        assert_eq!(topscorers[1].player_id, 72813);
        assert_eq!(topscorers[2].player_id, 163409);
        
        assert_eq!(topscorers[0].team_id, 349);
        assert_eq!(topscorers[1].team_id, 349);
        assert_eq!(topscorers[2].team_id, 3599);

        assert_eq!(topscorers[0].stage_id, Some(7664692));
        assert_eq!(topscorers[1].stage_id, Some(7664692));
        assert_eq!(topscorers[2].stage_id, Some(7664692));

        assert_eq!(topscorers[0].yellowcards, Some(2));
        assert_eq!(topscorers[1].yellowcards, Some(1));
        assert_eq!(topscorers[2].yellowcards, Some(1));

        assert_eq!(topscorers[0].redcards, Some(1));
        assert_eq!(topscorers[1].redcards, Some(1));
        assert_eq!(topscorers[2].redcards, Some(1));        
        
        assert_eq!(&topscorers[0].kind, "cards");
        assert_eq!(&topscorers[1].kind, "cards");
        assert_eq!(&topscorers[2].kind, "cards");
    }

    #[test]
    fn it_returns_a_single_season_with_goalscorers() {
        let body = fs::read_to_string(Path::new("src/support/seasons/find_with_goalscorers.json")).expect("Fixtures:");
        let m = mock("GET", "/seasons/12950?api_token=1234&include=goalscorers")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = SeasonGateway::new(Gateway::new("1234".into()));
        let response = instance.find_with(12950, Options::builder().include(&vec!["goalscorers"]));
        m.assert();
        
        assert!(response.is_ok());
        
        let result = response.unwrap();
        assert_eq!(result.data.id, 12950);
        assert_eq!(&result.data.name, "2018/2019");
        assert_eq!(result.data.league_id, 2);
        assert_eq!(result.data.is_current_season, true);
        assert_eq!(result.data.current_round_id, None);
        assert_eq!(result.data.current_stage_id, Some(7743657));
        assert_eq!(result.data.rounds, None);
        assert_eq!(result.data.stages, None);
        assert_eq!(result.data.upcoming, None);
        assert_eq!(result.data.results, None);
        assert_eq!(result.data.groups, None);
        assert_eq!(result.data.assistscorers, None);
        assert_eq!(result.data.aggregated_assistscorers, None);
        assert_eq!(result.data.aggregated_cardscorers, None);

        let topscorers = result.data.goalscorers.clone().unwrap();
        assert_eq!(topscorers[0].position, 1);
        assert_eq!(topscorers[1].position, 2);
        assert_eq!(topscorers[2].position, 3);
        
        assert_eq!(topscorers[0].season_id, 12950);
        assert_eq!(topscorers[1].season_id, 12950);
        assert_eq!(topscorers[2].season_id, 12950);

        assert_eq!(topscorers[0].player_id, 177983);
        assert_eq!(topscorers[1].player_id, 72614);
        assert_eq!(topscorers[2].player_id, 96792);
        
        assert_eq!(topscorers[0].team_id, 2673);
        assert_eq!(topscorers[1].team_id, 674);
        assert_eq!(topscorers[2].team_id, 53);

        assert_eq!(topscorers[0].stage_id, Some(7664692));
        assert_eq!(topscorers[1].stage_id, Some(7664692));
        assert_eq!(topscorers[2].stage_id, Some(7664692));

        assert_eq!(topscorers[0].goals, 3);
        assert_eq!(topscorers[1].goals, 2);
        assert_eq!(topscorers[2].goals, 2);

        assert_eq!(topscorers[0].penalty_goals, 0);
        assert_eq!(topscorers[1].penalty_goals, 0);
        assert_eq!(topscorers[2].penalty_goals, 0);        
        
        assert_eq!(&topscorers[0].kind, "goals");
        assert_eq!(&topscorers[1].kind, "goals");
        assert_eq!(&topscorers[2].kind, "goals");
    }
    
    #[test]
    fn it_returns_a_single_season_with_groups() {
        let body = fs::read_to_string(Path::new("src/support/seasons/find_with_groups.json")).expect("Fixtures:");
        let m = mock("GET", "/seasons/12950?api_token=1234&include=groups")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = SeasonGateway::new(Gateway::new("1234".into()));
        let response = instance.find_with(12950, Options::builder().include(&vec!["groups"]));
        m.assert();
        
        assert!(response.is_ok());
        
        let result = response.unwrap();
        assert_eq!(result.data.id, 12950);
        assert_eq!(&result.data.name, "2018/2019");
        assert_eq!(result.data.league_id, 2);
        assert_eq!(result.data.is_current_season, true);
        assert_eq!(result.data.current_round_id, None);
        assert_eq!(result.data.current_stage_id, Some(7743657));
        assert_eq!(result.data.rounds, None);
        assert_eq!(result.data.stages, None);
        assert_eq!(result.data.upcoming, None);
        assert_eq!(result.data.results, None);
        assert_eq!(result.data.assistscorers, None);
        assert_eq!(result.data.aggregated_assistscorers, None);
        assert_eq!(result.data.aggregated_cardscorers, None);

        let groups = result.data.groups.clone().unwrap();
        assert_eq!(groups[0].id, 83416);
        assert_eq!(&groups[0].name, "Group A");
        assert_eq!(groups[0].league_id, 2);
        assert_eq!(groups[0].season_id, 12950);
        assert_eq!(groups[0].round_id, None);
        assert_eq!(groups[0].round_name, None);
        assert_eq!(groups[0].stage_id, 7416759);
        assert_eq!(&groups[0].stage_name, "Group Stage");
    }

    #[test]
    fn it_returns_a_single_season_with_results() {
        let body = fs::read_to_string(Path::new("src/support/seasons/find_with_results.json")).expect("Fixtures:");
        let m = mock("GET", "/seasons/12950?api_token=1234&include=results")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = SeasonGateway::new(Gateway::new("1234".into()));
        let response = instance.find_with(12950, Options::builder().include(&vec!["results"]));
        m.assert();
        
        assert!(response.is_ok());
        
        let result = response.unwrap();
        assert_eq!(result.data.id, 12950);
        assert_eq!(&result.data.name, "2018/2019");
        assert_eq!(result.data.league_id, 2);
        assert_eq!(result.data.is_current_season, true);
        assert_eq!(result.data.current_round_id, None);
        assert_eq!(result.data.current_stage_id, Some(7743657));
        assert_eq!(result.data.rounds, None);
        assert_eq!(result.data.stages, None);
        assert_eq!(result.data.upcoming, None);
        assert_eq!(result.data.groups, None);
        assert_eq!(result.data.assistscorers, None);
        assert_eq!(result.data.aggregated_assistscorers, None);
        assert_eq!(result.data.aggregated_cardscorers, None);

        let results = result.data.results.clone().unwrap();
        assert_eq!(results[0].id, 10332447);
        assert_eq!(results[0].league_id, 2);
        assert_eq!(results[0].season_id, 12950);
        assert_eq!(results[0].stage_id, 7743435);
        assert_eq!(results[0].round_id, None);
        assert_eq!(results[0].group_id, None);
        assert_eq!(results[0].aggregate_id, None);
        assert_eq!(results[0].venue_id, Some(15038));
        assert_eq!(results[0].referee_id, Some(32931));
        assert_eq!(results[0].localteam_id, 7058);
        assert_eq!(results[0].visitorteam_id, 5292);

        let weather_report = results[0].clone().weather_report.unwrap();
        assert_eq!(&weather_report.code, "clear");
        assert_eq!(&weather_report.kind, "clear sky");
        assert_eq!(weather_report.temperature.temp, 79.0);
        assert_eq!(&weather_report.temperature.unit, "fahrenheit");
        assert_eq!(&weather_report.clouds, "0%");
        assert_eq!(&weather_report.humidity, "54%");
        assert_eq!(&weather_report.wind.speed, "12.75 m/s");
        assert_eq!(weather_report.wind.degree, Some(190.0));

        assert_eq!(results[0].commentaries, None);
        assert_eq!(results[0].attendance, None);
        assert_eq!(results[0].pitch, Some("Artificial Turf".to_string()));
        assert_eq!(results[0].winning_odds_calculated, true);

        assert_eq!(results[0].formations.localteam_formation, None);
        assert_eq!(results[0].formations.visitorteam_formation, None);

        assert_eq!(results[0].scores.localteam_score, 0);
        assert_eq!(results[0].scores.visitorteam_score, 2);
        assert_eq!(results[0].scores.localteam_pen_score, None);
        assert_eq!(results[0].scores.visitorteam_pen_score, None);
        assert_eq!(results[0].scores.ht_score, Some("0-0".to_string()));
        assert_eq!(results[0].scores.ft_score, Some("0-0".to_string()));
        assert_eq!(results[0].scores.et_score, Some("0-2".to_string()));

        assert_eq!(&results[0].time.status, "AET");
        assert_eq!(results[0].time.starting_at.date_time, "2018-06-26 15:00:00");
        assert_eq!(results[0].time.starting_at.date, "2018-06-26");
        assert_eq!(results[0].time.starting_at.time, "15:00:00");
        assert_eq!(results[0].time.starting_at.timestamp, 1530025200);
        assert_eq!(results[0].time.starting_at.timezone, "UTC");

        assert_eq!(results[0].time.minute, Some(120));
        assert_eq!(results[0].time.second, None);
        assert_eq!(results[0].time.added_time, Some(0));
        assert_eq!(results[0].time.extra_minute, None);
        assert_eq!(results[0].time.injury_time, Some(1));

        assert_eq!(results[0].coaches.localteam_coach_id, Some(465028));
        assert_eq!(results[0].coaches.visitorteam_coach_id, Some(12844179));

        assert_eq!(results[0].standings.localteam_position, None);
        assert_eq!(results[0].standings.visitorteam_position, None);
        
        assert_eq!(results[0].assistants.first_assistant_id, Some(48498));
        assert_eq!(results[0].assistants.second_assistant_id, Some(34682));
        assert_eq!(results[0].assistants.fourth_official_id, Some(44212));

        assert_eq!(results[0].leg, Some("1/1".to_string()));
        assert_eq!(results[0].colors, None);
        assert_eq!(results[0].deleted, false);

    }

     #[test]
    fn it_returns_a_single_season_with_rounds() {
        let body = fs::read_to_string(Path::new("src/support/seasons/find_with_rounds.json")).expect("Fixtures:");
        let m = mock("GET", "/seasons/12950?api_token=1234&include=rounds")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = SeasonGateway::new(Gateway::new("1234".into()));
        let response = instance.find_with(12950, Options::builder().include(&vec!["rounds"]));
        m.assert();
        
        assert!(response.is_ok());
        
        let result = response.unwrap();
        assert_eq!(result.data.id, 12950);
        assert_eq!(&result.data.name, "2018/2019");
        assert_eq!(result.data.league_id, 2);
        assert_eq!(result.data.is_current_season, true);
        assert_eq!(result.data.current_round_id, None);
        assert_eq!(result.data.current_stage_id, Some(7743657));
        assert_eq!(result.data.stages, None);
        assert_eq!(result.data.upcoming, None);
        assert_eq!(result.data.groups, None);
        assert_eq!(result.data.assistscorers, None);
        assert_eq!(result.data.aggregated_assistscorers, None);
        assert_eq!(result.data.aggregated_cardscorers, None);

        let rounds = result.data.rounds.clone().unwrap();
        assert_eq!(rounds[0].id, 155603);
        assert_eq!(rounds[0].name, 1);
        assert_eq!(rounds[0].league_id, 2);
        assert_eq!(rounds[0].season_id, 12950);
        assert_eq!(rounds[0].stage_id, 7416759);
        assert_eq!(&rounds[0].start, "2018-09-18");
        assert_eq!(&rounds[0].end, "2018-09-19");   
    }
    
    #[test]
    fn it_returns_a_single_season_with_stages() {
        let body = fs::read_to_string(Path::new("src/support/seasons/find_with_stages.json")).expect("Fixtures:");
        let m = mock("GET", "/seasons/12950?api_token=1234&include=stages")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = SeasonGateway::new(Gateway::new("1234".into()));
        let response = instance.find_with(12950, Options::builder().include(&vec!["stages"]));
        m.assert();
        
        assert!(response.is_ok());
        
        let result = response.unwrap();
        assert_eq!(result.data.id, 12950);
        assert_eq!(&result.data.name, "2018/2019");
        assert_eq!(result.data.league_id, 2);
        assert_eq!(result.data.is_current_season, true);
        assert_eq!(result.data.current_round_id, None);
        assert_eq!(result.data.current_stage_id, Some(7743657));
        assert_eq!(result.data.upcoming, None);
        assert_eq!(result.data.groups, None);
        assert_eq!(result.data.assistscorers, None);
        assert_eq!(result.data.aggregated_assistscorers, None);
        assert_eq!(result.data.aggregated_cardscorers, None);

        let stage = result.data.stages.clone().unwrap();
        assert_eq!(stage[0].id, 7743654);
        assert_eq!(&stage[0].name, "Final");
        assert_eq!(&stage[0].kind, "Knock Out");
        assert_eq!(stage[0].league_id, 2);
        assert_eq!(stage[0].season_id, 12950);
    }
    
    #[test]
    fn it_returns_a_single_season_with_upcoming() {
        let body = fs::read_to_string(Path::new("src/support/seasons/find_with_upcoming.json")).expect("Fixtures:");
        let m = mock("GET", "/seasons/12950?api_token=1234&include=upcoming")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = SeasonGateway::new(Gateway::new("1234".into()));
        let response = instance.find_with(12950, Options::builder().include(&vec!["upcoming"]));
        m.assert();
        assert!(response.is_ok());
        
        let result = response.unwrap();
        assert_eq!(result.data.id, 12950);
        assert_eq!(&result.data.name, "2018/2019");
        assert_eq!(result.data.league_id, 2);
        assert_eq!(result.data.is_current_season, true);
        assert_eq!(result.data.current_round_id, None);
        assert_eq!(result.data.current_stage_id, Some(7743657));
        assert_eq!(result.data.rounds, None);
        assert_eq!(result.data.stages, None);
        assert_eq!(result.data.groups, None);
        assert_eq!(result.data.assistscorers, None);
        assert_eq!(result.data.aggregated_assistscorers, None);
        assert_eq!(result.data.aggregated_cardscorers, None);

        let results = result.data.upcoming.clone().unwrap();
        assert_eq!(results[0].id, 11414776);
        assert_eq!(results[0].league_id, 2);
        assert_eq!(results[0].season_id, 12950);
        assert_eq!(results[0].stage_id, 7743657);
        assert_eq!(results[0].round_id, None);
        assert_eq!(results[0].group_id, None);
        assert_eq!(results[0].aggregate_id, Some(17561));
        assert_eq!(results[0].venue_id, Some(2085));
        assert_eq!(results[0].referee_id, None);
        assert_eq!(results[0].localteam_id, 67);
        assert_eq!(results[0].visitorteam_id, 9);
        assert_eq!(results[0].weather_report, None);

        assert_eq!(results[0].commentaries, Some(false));
        assert_eq!(results[0].attendance, None);
        assert_eq!(results[0].pitch, None);
        assert_eq!(results[0].winning_odds_calculated, false);

        assert_eq!(results[0].formations.localteam_formation, None);
        assert_eq!(results[0].formations.visitorteam_formation, None);

        assert_eq!(results[0].scores.localteam_score, 0);
        assert_eq!(results[0].scores.visitorteam_score, 0);
        assert_eq!(results[0].scores.localteam_pen_score, None);
        assert_eq!(results[0].scores.visitorteam_pen_score, None);
        assert_eq!(results[0].scores.ht_score, None);
        assert_eq!(results[0].scores.ft_score, None);
        assert_eq!(results[0].scores.et_score, None);

        assert_eq!(&results[0].time.status, "NS");
        assert_eq!(results[0].time.starting_at.date_time, "2019-02-20 20:00:00");
        assert_eq!(results[0].time.starting_at.date, "2019-02-20");
        assert_eq!(results[0].time.starting_at.time, "20:00:00");
        assert_eq!(results[0].time.starting_at.timestamp, 1550692800);
        assert_eq!(results[0].time.starting_at.timezone, "UTC");

        assert_eq!(results[0].time.minute, None);
        assert_eq!(results[0].time.second, None);
        assert_eq!(results[0].time.added_time, None);
        assert_eq!(results[0].time.extra_minute, None);
        assert_eq!(results[0].time.injury_time, None);

        assert_eq!(results[0].coaches.localteam_coach_id, Some(893655));
        assert_eq!(results[0].coaches.visitorteam_coach_id, Some(455361));

        assert_eq!(results[0].standings.localteam_position, None);
        assert_eq!(results[0].standings.visitorteam_position, None);
        
        assert_eq!(results[0].assistants.first_assistant_id, None);
        assert_eq!(results[0].assistants.second_assistant_id, None);
        assert_eq!(results[0].assistants.fourth_official_id, None);

        assert_eq!(results[0].leg, Some("1/2".to_string()));
        assert_eq!(results[0].colors, None);
        assert_eq!(results[0].deleted, false);
    }

    #[test]
    fn it_returns_error_if_season_does_not_exist() {
        let m = mock("GET", "/seasons/1234?api_token=1234")
          .with_status(403)
          .with_body("{\"error\":{\"message\":\"yadda yadda\",\"code\":403}}")
          .create();
        
        let instance = SeasonGateway::new(Gateway::new("1234".into()));
        let response = instance.find(1234);
        m.assert();
        
        assert!(response.is_err());
        let sportmonks_error = response.unwrap_err();
        assert_eq!(sportmonks_error.message(), "yadda yadda".to_string());
        assert_eq!(sportmonks_error.code(), 403);
    }
}