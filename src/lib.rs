extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
#[cfg(test)]
extern crate mockito;
extern crate chrono;

pub mod ops;
pub mod models;
pub mod errors;
pub mod gateway;

use gateway::{Gateway};

pub struct Client {
    pub contients: ops::ContinentGateway,
    pub commentaries: ops::CommentariesGateway,
    pub countries: ops::CountryGateway,
    pub fixtures: ops::FixtureGateway,
    pub head_to_head: ops::HeadToHeadGateway,
    pub leagues: ops::LeagueGateway,
    pub livescores: ops::LivescoreGateway,
    pub players: ops::PlayerGateway,
    pub seasons: ops::SeasonGateway,
    pub standings: ops::StandingGateway,
    pub teams: ops::TeamGateway,
    pub topscorers: ops::TopscorerGateway,

}

impl Client {
    pub fn new<S: Into<String>>(api_key: S) -> Client {
        let api_key_string = api_key.into();
        Client {
            contients: ops::ContinentGateway::new(Gateway::new(api_key_string.clone())),
            commentaries: ops::CommentariesGateway::new(Gateway::new(api_key_string.clone())),
            countries: ops::CountryGateway::new(Gateway::new(api_key_string.clone())),
            fixtures: ops::FixtureGateway::new(Gateway::new(api_key_string.clone())),
            head_to_head: ops::HeadToHeadGateway::new(Gateway::new(api_key_string.clone())),
            leagues: ops::LeagueGateway::new(Gateway::new(api_key_string.clone())),
            livescores: ops::LivescoreGateway::new(Gateway::new(api_key_string.clone())),
            players: ops::PlayerGateway::new(Gateway::new(api_key_string.clone())),
            seasons: ops::SeasonGateway::new(Gateway::new(api_key_string.clone())),
            standings: ops::StandingGateway::new(Gateway::new(api_key_string.clone())),
            teams: ops::TeamGateway::new(Gateway::new(api_key_string.clone())),
            topscorers: ops::TopscorerGateway::new(Gateway::new(api_key_string.clone())),
        }
    }
}
