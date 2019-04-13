use super::super::errors::SportMonksError;
use super::super::models::{Team,Wrapper};
use super::super::gateway::{Gateway,Options};


pub struct TeamGateway {
    gateway: Gateway
}

impl TeamGateway {
    pub fn new(gateway: Gateway) -> TeamGateway {
        TeamGateway { gateway }
    }
    
    pub fn find(&self, id: i64) -> Result<Wrapper<Team>, SportMonksError> {
        self.find_with(id, Options::empty())
    }

    pub fn find_with(&self, id: i64, options: Options) -> Result<Wrapper<Team>, SportMonksError> {
        let path = format!("/teams/{}", id);
        self.gateway.get(&path, options)
    }

    pub fn of_season(&self, season_id: i64) -> Result<Wrapper<Vec<Team>>, SportMonksError> {
        self.of_season_with(season_id, Options::empty())
    }

    pub fn of_season_with(&self, season_id: i64, options: Options) -> Result<Wrapper<Vec<Team>>, SportMonksError> {
        let path = format!("/teams/season/{}", season_id);
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
    fn it_returns_a_given_team() {
        let body = fs::read_to_string(Path::new("src/support/teams/find_with.json")).expect("Fixtures:");
        let m = mock("GET", "/teams/9?api_token=1234&include=country%2Csquad%2Ccoach%2Ctransfers%2Csidelined%2Cstats%2Cvenue%2Cuefaranking%2CvisitorFixtures%2ClocalFixtures%2CvisitorResults%2Clatest%2Cupcoming%2Cgoalscorers%2Ccardscorers%2Cassistscorers%2CaggregatedGoalscorers%2CaggregatedCardscorers%2CaggregatedAssistscorers")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = TeamGateway::new(Gateway::new("1234".into()));
        let includes = vec!["country,squad,coach,transfers,sidelined,stats,venue,uefaranking,visitorFixtures,localFixtures,visitorResults,latest,upcoming,goalscorers,cardscorers,assistscorers,aggregatedGoalscorers,aggregatedCardscorers,aggregatedAssistscorers"];
        let options = Options::builder().include(&includes);
        let team = instance.find_with(9, options).unwrap().data;
        m.assert();

        assert_eq!(team.id, 9);
        assert!(team.country.is_some());
        assert!(team.squad.is_some());
        assert!(team.coach.is_some());
        assert!(team.transfers.is_some());
        assert!(team.sidelined.is_some());
        assert!(team.venue.is_some());
        assert!(team.uefaranking.is_some());
        assert!(team.visitor_fixtures.is_some());
        assert!(team.local_fixtures.is_some());
        assert!(team.visitor_results.is_some());
        assert!(team.latest.is_some());
        assert!(team.upcoming.is_some());
        assert!(team.goalscorers.is_some());
        assert!(team.cardscorers.is_some());
        assert!(team.assistscorers.is_some());
        assert!(team.aggregated_goalscorers.is_some());
        assert!(team.aggregated_cardscorers.is_some());
        assert!(team.aggregated_assistscorers.is_some());
    }
    
    #[test]
    fn it_returns_teams_which_belong_to_a_season() {
        let body = fs::read_to_string(Path::new("src/support/teams/on_season.json")).expect("Fixtures:");
        let m = mock("GET", "/teams/season/12962?api_token=1234")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = TeamGateway::new(Gateway::new("1234".into()));
        let team = instance.of_season(12962).unwrap().data[0].clone();
        m.assert();
        assert_eq!(team.id, 1);
        assert_eq!(team.legacy_id, Some(377));
        assert_eq!(&team.name, "West Ham United");
        assert_eq!(team.short_code, Some("WHU".into()));
        assert_eq!(team.twitter, Some("@WestHamUtd".to_string()));
        assert_eq!(team.country_id, 462);
        assert_eq!(team.national_team, false);
        assert_eq!(team.founded, Some(1895));
        assert_eq!(team.logo_path, Some("https://cdn.sportmonks.com/images/soccer/teams/1/1.png".into()));
        assert_eq!(team.venue_id, Some(214));
        assert_eq!(team.current_season_id, Some(12962));
    }
    
    #[test]
    fn it_returns_teams_which_belong_to_a_season_with_extra_info() {
        let body = fs::read_to_string(Path::new("src/support/teams/of_season_with.json")).expect("Fixtures:");
        let m = mock("GET", "/teams/season/12962?api_token=1234&include=squad")
          .with_status(200)
          .with_body(body)
          .create();
        
        let options = Options::builder().include(&["squad"]);
        let instance = TeamGateway::new(Gateway::new("1234".into()));
        let result = instance.of_season_with(12962, options);

        m.assert();
        assert!(result.is_ok());
    }

    #[test]
       fn it_works_with_double_include() {
        let body = fs::read_to_string(Path::new("src/support/seasons/double_nested.json")).expect("Fixtures:");
        let m = mock("GET", "/teams/season/12950?api_token=1234&include=squad.player")
          .with_status(200)
          .with_body(body)
          .create();
        
        let options = Options::builder().include(&["squad.player"]);
        let instance = TeamGateway::new(Gateway::new("1234".into()));
        let result = instance.of_season_with(12950, options);

        m.assert();
        println!("{:?}", result);
        assert!(result.is_ok());
    }

}