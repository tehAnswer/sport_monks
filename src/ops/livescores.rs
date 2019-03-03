use super::super::errors::SportMonksError;
use super::super::models::{Fixture,Wrapper};
use super::super::gateway::{Gateway,Options};

pub struct LivescoreGateway {
    gateway: Gateway
}

impl LivescoreGateway {
    pub fn new(gateway: Gateway) -> LivescoreGateway {
        LivescoreGateway { gateway }
    }

    pub fn all(&self) -> Result<Wrapper<Vec<Fixture>>, SportMonksError> {
        self.gateway.get("/livescores", Options::empty())
    }

    pub fn all_with(&self, options: Options) -> Result<Wrapper<Vec<Fixture>>, SportMonksError> {
        self.gateway.get("/livescores", options)
    }

    pub fn now(&self) -> Result<Wrapper<Vec<Fixture>>, SportMonksError> {
        self.gateway.get("/livescores/now", Options::empty())
    }

    pub fn now_with(&self, options: Options) -> Result<Wrapper<Vec<Fixture>>, SportMonksError> {
        self.gateway.get("/livescores/now", options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;
    use std::fs;
    use std::path::Path;

    #[test]
    fn it_returns_all_the_livescores_with_details() {
        let body = fs::read_to_string(Path::new("src/support/livescores/all_with.json")).expect("Fixtures:");
        let m = mock("GET", "/livescores?api_token=1234&include=localTeam%2CvisitorTeam")
          .with_status(200)
          .with_body(body)
          .create();

        let instance = LivescoreGateway::new(Gateway::new("1234".into()));
        let response = instance.all_with(Options::builder().include(&vec!["localTeam,visitorTeam"]));
        m.assert();


        assert!(response.is_ok());
        let fixtures = response.unwrap().data;
        assert_eq!(fixtures[0].id, 10420302);
        assert_eq!(fixtures[0].league_id, 384);
        assert_eq!(fixtures[0].season_id, 13158);
        assert_eq!(fixtures[0].round_id, Some(151888));
        assert_eq!(fixtures[0].group_id, None);
        assert_eq!(fixtures[0].aggregate_id, None);
        assert_eq!(fixtures[0].venue_id, Some(7219));
        assert_eq!(fixtures[0].referee_id, Some(15306));
        assert_eq!(fixtures[0].localteam_id, 37);
        assert_eq!(fixtures[0].visitorteam_id, 8513);
        assert_eq!(fixtures[0].weather_report, None);
        assert_eq!(fixtures[0].commentaries, Some(false));
        assert_eq!(fixtures[0].attendance, None);
        assert_eq!(fixtures[0].pitch, None);
        assert_eq!(fixtures[0].winning_odds_calculated, false);
        assert_eq!(fixtures[0].formations.localteam_formation, None);
        assert_eq!(fixtures[0].formations.visitorteam_formation, None);
        assert_eq!(fixtures[0].scores.localteam_score, 0);
        assert_eq!(fixtures[0].scores.visitorteam_score, 0);
        assert_eq!(fixtures[0].scores.localteam_pen_score, Some(0));
        assert_eq!(fixtures[0].scores.visitorteam_pen_score, Some(0));
        assert_eq!(fixtures[0].scores.ht_score, None);
        assert_eq!(fixtures[0].scores.ft_score, None);
        assert_eq!(fixtures[0].scores.et_score, None);

        let local_team = fixtures[0].clone().local_team.unwrap();
        assert_eq!(local_team.id, 37);
        assert_eq!(local_team.legacy_id, 128);
        assert_eq!(&local_team.name, "Roma");
        assert_eq!(&local_team.short_code, "ROM");
        assert_eq!(local_team.twitter, Some("@OfficialASRoma".to_string()));
        assert_eq!(local_team.country_id, 251);
        assert_eq!(local_team.national_team, false);
        assert_eq!(local_team.founded, 1927);
        assert_eq!(&local_team.logo_path, "https://cdn.sportmonks.com/images/soccer/teams/5/37.png");
        assert_eq!(local_team.venue_id, 7219);
        assert_eq!(local_team.current_season_id, None);

        let visitor_team = fixtures[0].clone().visitor_team.unwrap();
        assert_eq!(visitor_team.id, 8513);
        assert_eq!(visitor_team.legacy_id, 338);
        assert_eq!(&visitor_team.name, "Bologna");
        assert_eq!(&visitor_team.short_code, "BGN");
        assert_eq!(visitor_team.twitter, Some("@BfcOfficialPage".to_string()));
        assert_eq!(visitor_team.country_id, 251);
        assert_eq!(visitor_team.national_team, false);
        assert_eq!(visitor_team.founded, 1909);
        assert_eq!(&visitor_team.logo_path, "https://cdn.sportmonks.com/images/soccer/teams/1/8513.png");
        assert_eq!(visitor_team.venue_id, 5515);
        assert_eq!(visitor_team.current_season_id, None);
    }

    #[test]
    fn it_returns_all_the_livescores_in_play_with_details() {
        let body = fs::read_to_string(Path::new("src/support/livescores/now_with.json")).expect("Fixtures:");
        let m = mock("GET", "/livescores/now?api_token=1234&include=inplay")
          .with_status(200)
          .with_body(body)
          .create();

        let instance = LivescoreGateway::new(Gateway::new("1234".into()));
        let response = instance.now_with(Options::builder().include(&vec!["inplay"]));
        m.assert();

        assert!(response.is_ok());
        let fixtures = response.unwrap().data;
        assert!(fixtures[0].inplay.is_some());
    }

}
