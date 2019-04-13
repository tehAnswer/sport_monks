use super::super::errors::SportMonksError;
use super::super::models::{Fixture,Wrapper};
use super::super::gateway::{Gateway,Options};



pub struct HeadToHeadGateway {
    gateway: Gateway
}

impl HeadToHeadGateway {
    pub fn new(gateway: Gateway) -> HeadToHeadGateway {
        HeadToHeadGateway { gateway }
    }
    pub fn find(&self, team_one_id: i64, team_two_id: i64) -> Result<Wrapper<Vec<Fixture>>, SportMonksError> {
        let path = format!("/head2head/{}/{}", team_one_id, team_two_id);
        self.gateway.get(&path, Options::empty())
    }
    
    pub fn find_with(&self, team_one_id: i64, team_two_id: i64, options: Options) -> Result<Wrapper<Vec<Fixture>>, SportMonksError> {
        let path = format!("/head2head/{}/{}", team_one_id, team_two_id);
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
    fn it_finds_fixtures_by_id() {
        let body = fs::read_to_string(Path::new("src/support/head2head/find.json")).expect("Fixtures:");
        let m = mock("GET", "/head2head/67/9?api_token=1234")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = HeadToHeadGateway::new(Gateway::new("1234".into()));
        let fixtures = instance.find(67, 9).unwrap().data;
        m.assert();

        assert_eq!(fixtures[0].id, 1061771);
        assert_eq!(fixtures[0].league_id, 5);
        assert_eq!(fixtures[0].season_id, 5329);
        assert_eq!(fixtures[0].round_id, None);
        assert_eq!(fixtures[0].group_id, Some(226791));
    }
}

