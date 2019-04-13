use super::super::errors::SportMonksError;
use super::super::models::{League,Wrapper};
use super::super::gateway::{Gateway,Options};


pub struct LeagueGateway {
    gateway: Gateway
}

impl LeagueGateway {
    pub fn new(gateway: Gateway) -> LeagueGateway {
        LeagueGateway { gateway }
    }

    pub fn all(&self) -> Result<Wrapper<Vec<League>>, SportMonksError> {
        self.gateway.get("/leagues", Options::empty())
    }
    
    pub fn all_with(&self, options: Options) -> Result<Wrapper<Vec<League>>, SportMonksError> {
        self.gateway.get("/leagues", options)
    }
    
    pub fn find(&self, id: i64) -> Result<Wrapper<League>, SportMonksError> {
        let path = format!("/leagues/{}", id);
        self.gateway.get(&path, Options::empty())
    }

    pub fn find_with(&self, id: i64, options: Options) -> Result<Wrapper<League>, SportMonksError> {
        let path = format!("/leagues/{}", id);
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
    fn it_returns_all_the_leagues() {
        let body = fs::read_to_string(Path::new("src/support/leagues/all.json")).expect("Fixtures:");
        let m = mock("GET", "/leagues?api_token=1234")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = LeagueGateway::new(Gateway::new("1234".into()));
        let response = instance.all();
        m.assert();

        assert!(response.is_ok());
        let leagues = response.unwrap().data;
        assert_eq!(leagues.len(), 8);
        assert_eq!(leagues[0].id, 2);
        assert_eq!(&leagues[0].name, "Champions League");
        assert_eq!(leagues[0].legacy_id, 11);
        assert_eq!(leagues[0].country_id, 41);
        assert_eq!(leagues[0].is_cup, true);
        assert_eq!(leagues[0].current_season_id, 12950);
        assert_eq!(leagues[0].current_round_id, None);
        assert_eq!(leagues[0].current_stage_id, 7743657);
        assert_eq!(leagues[0].live_standings, true);
        assert_eq!(leagues[0].coverage.topscorer_goals, true);
        assert_eq!(leagues[0].coverage.topscorer_assists, true);
        assert_eq!(leagues[0].coverage.topscorer_cards, true);
    }

    #[test]
    fn it_returns_all_the_leagues_with_details() {
        let body = fs::read_to_string(Path::new("src/support/leagues/all_with.json")).expect("Fixtures:");
        let m = mock("GET", "/leagues?api_token=1234&include=country%2Cseason%2Cseasons%3Afilter%281%7C2%29")
          .with_status(200)
          .with_body(body)
          .create();

        let instance = LeagueGateway::new(Gateway::new("1234".into()));
        let opts = Options::builder().include(&vec!["country", "season", "seasons:filter(1|2)"]);
        let response = instance.all_with(opts);
        m.assert();

        assert!(response.is_ok());
        let leagues = response.unwrap().data;
        assert_eq!(leagues.len(), 8);
        assert_eq!(leagues[0].id, 2);
        assert_eq!(&leagues[0].name, "Champions League");
        assert_eq!(leagues[0].legacy_id, 11);
        assert_eq!(leagues[0].country_id, 41);
        assert_eq!(leagues[0].is_cup, true);
        assert_eq!(leagues[0].current_season_id, 12950);
        assert_eq!(leagues[0].current_round_id, None);
        assert_eq!(leagues[0].current_stage_id, 7743657);
        assert_eq!(leagues[0].live_standings, true);
        assert_eq!(leagues[0].coverage.topscorer_goals, true);
        assert_eq!(leagues[0].coverage.topscorer_assists, true);
        assert_eq!(leagues[0].coverage.topscorer_cards, true);
        assert!(leagues[0].country.is_some());

        let league_country = leagues[0].country.clone().unwrap();
        assert_eq!(league_country.id, 41);
        assert_eq!(league_country.name, "Europe");
        assert_eq!(league_country.extra, None);

        let league_current_season = leagues[0].season.clone().unwrap();
        assert_eq!(league_current_season.id, 12950);
        assert_eq!(&league_current_season.name, "2018/2019");
        assert_eq!(league_current_season.league_id, 2);
        assert_eq!(league_current_season.is_current_season, true);
        assert_eq!(league_current_season.current_round_id, None);
        assert_eq!(league_current_season.current_stage_id, Some(7743657));

        let league_seasons = leagues[0].seasons.clone().unwrap();
        assert_eq!(league_seasons.len(), 0);
    }

    #[test]
    fn it_returns_a_single_league() {
        let body = fs::read_to_string(Path::new("src/support/leagues/find.json")).expect("Fixtures:");
        let m = mock("GET", "/leagues/2?api_token=1234")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = LeagueGateway::new(Gateway::new("1234".into()));
        let response = instance.find(2);
        m.assert();
        assert!(response.is_ok());
        let league = response.unwrap().data;
        assert_eq!(league.id, 2);
        assert_eq!(&league.name, "Champions League");
        assert_eq!(league.legacy_id, 11);
        assert_eq!(league.country_id, 41);
        assert_eq!(league.is_cup, true);
        assert_eq!(league.current_season_id, 12950);
        assert_eq!(league.current_round_id, None);
        assert_eq!(league.current_stage_id, 7743657);
        assert_eq!(league.live_standings, true);
        assert_eq!(league.coverage.topscorer_goals, true);
        assert_eq!(league.coverage.topscorer_assists, true);
        assert_eq!(league.coverage.topscorer_cards, true);
        assert_eq!(league.country, None);
        assert_eq!(league.season, None);
        assert_eq!(league.seasons, None);
    }

    #[test]
    fn it_returns_a_single_league_with_details() {
        let body = fs::read_to_string(Path::new("src/support/leagues/find_with.json")).expect("Fixtures:");
        let m = mock("GET", "/leagues/2?api_token=1234&include=country")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = LeagueGateway::new(Gateway::new("1234".into()));
        let response = instance.find_with(2, Options::builder().include(&vec!["country"]));
        m.assert();
        
        assert!(response.is_ok());
        let league = response.unwrap().data;
        assert_eq!(league.id, 2);
        assert_eq!(&league.name, "Champions League");
        assert_eq!(league.legacy_id, 11);
        assert_eq!(league.country_id, 41);
        assert_eq!(league.is_cup, true);
        assert_eq!(league.current_season_id, 12950);
        assert_eq!(league.current_round_id, None);
        assert_eq!(league.current_stage_id, 7743657);
        assert_eq!(league.live_standings, true);
        assert_eq!(league.coverage.topscorer_goals, true);
        assert_eq!(league.coverage.topscorer_assists, true);
        assert_eq!(league.coverage.topscorer_cards, true);
        assert_eq!(league.season, None);
        assert_eq!(league.seasons, None);
        assert!(league.country.is_some());
        
        let league_country = league.country.unwrap();
        assert_eq!(league_country.id, 41);
        assert_eq!(league_country.name, "Europe");
        assert_eq!(league_country.extra, None);
    }

    #[test]
    fn it_returns_error_if_league_does_not_exist() {
        let m = mock("GET", "/leagues/1234?api_token=1234")
          .with_status(403)
          .with_body("{\"error\":{\"message\":\"yadda yadda\",\"code\":403}}")
          .create();
        
        let instance = LeagueGateway::new(Gateway::new("1234".into()));
        let response = instance.find(1234);
        m.assert();
        
        assert!(response.is_err());
        let sportmonks_error = response.unwrap_err();
        assert_eq!(sportmonks_error.message(), "yadda yadda".to_string());
        assert_eq!(sportmonks_error.code(), 403);
    }

}