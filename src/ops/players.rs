use super::super::errors::SportMonksError;
use super::super::models::{Player,Wrapper};
use super::super::gateway::{Gateway,Options};


pub struct PlayerGateway {
    gateway: Gateway
}

impl PlayerGateway {
    pub fn new(gateway: Gateway) -> PlayerGateway {
        PlayerGateway { gateway }
    }

    pub fn find(&self, id: i64) -> Result<Wrapper<Player>, SportMonksError> {
        self.find_with(id, Options::empty())
    }

    pub fn find_with(&self, id: i64, options: Options) -> Result<Wrapper<Player>, SportMonksError> {
        let path = format!("/players/{}", id);
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
    fn it_returns_one_player() {
        let body = fs::read_to_string(Path::new("src/support/players/find_with.json")).expect("Fixtures:");
        let m = mock("GET", "/players/26759?api_token=1234&include=position%2Cstats%2Ctrophies%2Csidelined%2Ctransfers")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = PlayerGateway::new(Gateway::new("1234".into()));
        let options = Options::builder().include(&vec!["position,stats,trophies,sidelined,transfers"]);
        let response = instance.find_with(26759, options);
        m.assert();

        assert!(response.is_ok());
        let player = response.unwrap().data;

        assert!(player.position.is_some());
        assert!(player.trophies.is_some());
        assert!(player.sidelined.is_some());
        assert!(player.transfers.is_some());
    }
}