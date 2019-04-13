use super::super::errors::SportMonksError;
use super::super::models::{Standing,LiveStanding,Wrapper};
use super::super::gateway::{Gateway,Options};


pub struct StandingGateway {
    gateway: Gateway
}

impl StandingGateway {
    pub fn new(gateway: Gateway) -> StandingGateway {
        StandingGateway { gateway }
    }

    pub fn find(&self, season_id: i64) -> Result<Wrapper<Vec<Standing>>, SportMonksError> {
        self.find_with(season_id, Options::empty())
    }
    
    pub fn find_with(&self, season_id: i64, options: Options) -> Result<Wrapper<Vec<Standing>>, SportMonksError> {
        let path = format!("/standings/season/{}", season_id);
        self.gateway.get(&path, options)
    }

    pub fn live(&self, id: i64) -> Result<Wrapper<Vec<LiveStanding>>, SportMonksError> {
        let path = format!("/standings/season/live/{}", id);
        self.gateway.get(&path, Options::empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;
    use std::fs;
    use std::path::Path;

    #[test]
    fn it_returns_the_standings() {
        let body = fs::read_to_string(Path::new("src/support/standings/find.json")).expect("Fixtures:");
        let m = mock("GET", "/standings/season/12962?api_token=1234&include=standings.team")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = StandingGateway::new(Gateway::new("1234".into()));
        let options = Options::builder().include(&vec!["standings.team"]);
        let standings = instance.find_with(12962, options).unwrap().data;
        m.assert();

        assert_eq!(&standings[0].name, "Regular Season");
        assert_eq!(standings[0].league_id, 8);
        assert_eq!(standings[0].season_id, 12962);
        assert_eq!(standings[0].kind, Some("Group Stage".into()));
        assert_eq!(standings[0].stage_id, 7456626);
        assert_eq!(&standings[0].stage_name, "Regular Season");

        let first_team = standings[0].standings[0].clone();

        assert_eq!(first_team.position, 1);
        assert_eq!(first_team.team_id, 9);
        assert_eq!(&first_team.team_name, "Manchester City");
        assert_eq!(first_team.round_id, 147732);
        assert_eq!(first_team.round_name, 27);
        assert_eq!(first_team.group_id, None);
        assert_eq!(first_team.group_name, None);

        assert_eq!(first_team.overall.games_played, 27);
        assert_eq!(first_team.overall.won, 21);
        assert_eq!(first_team.overall.draw, 2);
        assert_eq!(first_team.overall.lost, 4);
        assert_eq!(first_team.overall.goals_scored, 74);
        assert_eq!(first_team.overall.goals_against, 20);

        assert_eq!(first_team.home.games_played, 14);
        assert_eq!(first_team.home.won, 13);
        assert_eq!(first_team.home.draw, 0);
        assert_eq!(first_team.home.lost, 1);
        assert_eq!(first_team.home.goals_scored, 49);
        assert_eq!(first_team.home.goals_against, 11);

        assert_eq!(first_team.away.games_played, 13);
        assert_eq!(first_team.away.won, 8);
        assert_eq!(first_team.away.draw, 2);
        assert_eq!(first_team.away.lost, 3);
        assert_eq!(first_team.away.goals_scored, 25);
        assert_eq!(first_team.away.goals_against, 9);

        assert_eq!(first_team.total.goal_difference, 54);
        assert_eq!(first_team.total.points, 65);

        assert_eq!(first_team.result, Some("Champions League".to_string()));
        assert_eq!(first_team.points, 65);
        assert_eq!(&first_team.recent_form, "WLWWW");
        assert_eq!(&first_team.status, "same");

        let team_info = standings[0].standings[0].team.clone().unwrap();
        assert_eq!(team_info.id, 9);
        assert_eq!(team_info.legacy_id, Some(127));
        assert_eq!(&team_info.name, "Manchester City");
        assert_eq!(team_info.short_code, Some("MCI".into()));
        assert_eq!(team_info.twitter, Some("@ManCity".to_string()));
        assert_eq!(team_info.country_id, 462);
        assert_eq!(team_info.national_team, false);
        assert_eq!(team_info.founded, Some(1880));
        assert_eq!(team_info.logo_path, Some("https://cdn.sportmonks.com/images/soccer/teams/9/9.png".into()));
        assert_eq!(team_info.venue_id, Some(151));
        assert_eq!(team_info.current_season_id, Some(12962));
    }


    #[test]
    fn it_returns_live_standings() {
        let body = fs::read_to_string(Path::new("src/support/standings/live.json")).expect("Fixtures:");
        let m = mock("GET", "/standing/season/live/12962?api_token=1234")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = StandingGateway::new(Gateway::new("1234".into()));
        let live_standings = instance.live(12962).unwrap().data;
        m.assert();

        assert_eq!(live_standings[0].position, 1);
        assert_eq!(live_standings[0].played, 27);
        assert_eq!(live_standings[0].team_id, 9);
        assert_eq!(&live_standings[0].team_name, "Manchester City");
        assert_eq!(&live_standings[0].short_code, "MCI");
        assert_eq!(&live_standings[0].team_logo, "https://cdn.sportmonks.com/images/soccer/teams/9/9.png");
        assert_eq!(&live_standings[0].goals, "74:20");
        assert_eq!(live_standings[0].goal_diff, 54);
        assert_eq!(live_standings[0].wins, 21);
        assert_eq!(live_standings[0].lost, 4);
        assert_eq!(live_standings[0].draws, 2);
        assert_eq!(live_standings[0].points, 65);
        assert_eq!(&live_standings[0].description, "Champions League");
        assert_eq!(live_standings[0].fairplay_points_lose, 38);
    }
}