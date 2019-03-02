use super::super::errors::SportMonksError;
use super::super::models::{Season,Wrapper};
use super::super::gateway::{Gateway,Options};


pub struct TopscorerGateway {
    gateway: Gateway
}

impl TopscorerGateway {
    pub fn new(gateway: Gateway) -> TopscorerGateway {
        TopscorerGateway { gateway }
    }
    

    pub fn of_season(&self, season_id: u64) -> Result<Wrapper<Season>, SportMonksError> {
        self.of_season_with(season_id, Options::empty())
    }

    pub fn of_season_with(&self, season_id: u64, options: Options) -> Result<Wrapper<Season>, SportMonksError> {
        let path = format!("/topscorers/season/{}", season_id);
        self.gateway.get(&path, options)
    }
    
    pub fn aggregated_of_season(&self, season_id: u64) -> Result<Wrapper<Season>, SportMonksError> {
        self.aggregated_of_season_with(season_id, Options::empty())
    }

    pub fn aggregated_of_season_with(&self, season_id: u64, options: Options) -> Result<Wrapper<Season>, SportMonksError> {
        let path = format!("/topscorers/season/{}/aggregated", season_id);
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
    fn it_returns_top_scorers_of_a_season() {
        let body = fs::read_to_string(Path::new("src/support/topscorers/by_season.json")).expect("Fixtures:");
        let m = mock("GET", "/topscorers/season/12950?api_token=1234")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = TopscorerGateway::new(Gateway::new("1234".into()));
        let season = instance.of_season(12950).unwrap().data;
        m.assert();

        let goalscorers = season.goalscorers.unwrap().clone();
        assert_eq!(goalscorers[0].position, 1);
        assert_eq!(goalscorers[0].season_id, 12950);
        assert_eq!(goalscorers[0].player_id, 177983);
        assert_eq!(goalscorers[0].team_id, 2673);
        assert_eq!(goalscorers[0].stage_id, Some(7664692));
        assert_eq!(goalscorers[0].goals, 3);
        assert_eq!(goalscorers[0].penalty_goals, 0);
        assert_eq!(&goalscorers[0].kind, "goals");

        let assistscorers = season.assistscorers.unwrap().clone();
        assert_eq!(assistscorers[0].position, 1);
        assert_eq!(assistscorers[0].season_id, 12950);
        assert_eq!(assistscorers[0].player_id, 67827);
        assert_eq!(assistscorers[0].team_id, 674);
        assert_eq!(assistscorers[0].stage_id, Some(7664692));
        assert_eq!(assistscorers[0].assists, Some(2));
        assert_eq!(assistscorers[0].kind, "assists");

        let cardscorers = season.cardscorers.unwrap().clone();
        assert_eq!(cardscorers[0].position, 1);
        assert_eq!(cardscorers[0].season_id, 12950);
        assert_eq!(cardscorers[0].player_id, 74038);
        assert_eq!(cardscorers[0].team_id, 349);
        assert_eq!(cardscorers[0].stage_id, Some(7664692));
        assert_eq!(cardscorers[0].yellowcards, Some(2));
        assert_eq!(cardscorers[0].redcards, Some(1));
        assert_eq!(cardscorers[0].kind, "cards");
    }

    #[test]
    fn it_returns_aggregated_top_scorers_of_a_season() {
        let body = fs::read_to_string(Path::new("src/support/topscorers/aggregated.json")).expect("Fixtures:");
        let m = mock("GET", "/topscorers/season/12950/aggregated?api_token=1234")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = TopscorerGateway::new(Gateway::new("1234".into()));
        let season = instance.aggregated_of_season(12950).unwrap().data;
        m.assert();

        let goalscorers = season.aggregated_goalscorers.unwrap().clone();
        assert_eq!(goalscorers[0].position, 144);
        assert_eq!(goalscorers[0].season_id, 12950);
        assert_eq!(goalscorers[0].player_id, 48532);
        assert_eq!(goalscorers[0].team_id, 724);
        assert_eq!(goalscorers[0].stage_id, None);
        assert_eq!(goalscorers[0].goals, 1);
        assert_eq!(goalscorers[0].penalty_goals, 0);
        assert_eq!(&goalscorers[0].kind, "aggregated_goals");

        let assistscorers = season.aggregated_assistscorers.unwrap().clone();
        assert_eq!(assistscorers[0].position, 1);
        assert_eq!(assistscorers[0].season_id, 12950);
        assert_eq!(assistscorers[0].player_id, 96611);
        assert_eq!(assistscorers[0].team_id, 591);
        assert_eq!(assistscorers[0].stage_id, None);
        assert_eq!(assistscorers[0].assists, Some(5));
        assert_eq!(assistscorers[0].kind, "aggregated_assists");

        let cardscorers = season.aggregated_cardscorers.unwrap().clone();
        assert_eq!(cardscorers[0].position, 1);
        assert_eq!(cardscorers[0].season_id, 12950);
        assert_eq!(cardscorers[0].player_id, 188444);
        assert_eq!(cardscorers[0].team_id, 629);
        assert_eq!(cardscorers[0].stage_id, None);
        assert_eq!(cardscorers[0].yellowcards, Some(7));
        assert_eq!(cardscorers[0].redcards, Some(0));
        assert_eq!(cardscorers[0].kind, "aggregated_cards");
    }
}