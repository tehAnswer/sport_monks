use super::super::errors::SportMonksError;
use super::super::models::{Country,Wrapper};
use super::super::gateway::{Gateway,Options};


pub struct CountryGateway {
    gateway: Gateway
}

impl CountryGateway {
    pub fn new(gateway: Gateway) -> CountryGateway {
        CountryGateway { gateway }
    }

    pub fn all(&self) -> Result<Wrapper<Vec<Country>>, SportMonksError> {
        self.gateway.get("/countries", Options::empty())
    }
    
    pub fn all_with(&self, options: Options) -> Result<Wrapper<Vec<Country>>, SportMonksError> {
        self.gateway.get("/countries", options)
    }
    
    pub fn find(&self, id: i64) -> Result<Wrapper<Country>, SportMonksError> {
        let path = format!("/countries/{}", id);
        self.gateway.get(&path, Options::empty())
    }

    pub fn find_with(&self, id: i64, options: Options) -> Result<Wrapper<Country>, SportMonksError> {
        let path = format!("/countries/{}", id);
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
    fn it_returns_all_the_countries() {
        let body = fs::read_to_string(Path::new("src/support/countries/all.json")).expect("Fixtures:");
        let m = mock("GET", "/countries?api_token=1234")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = CountryGateway::new(Gateway::new("1234".into()));
        let response = instance.all();
        m.assert();

        assert!(response.is_ok());
        let result = response.unwrap();
        assert_eq!(result.data[0].id, 11);
        assert_eq!(result.data[0].name, "Germany".to_string());
        assert_eq!(result.data[1].name, "Northern Ireland".to_string());
        assert!(result.data[0].extra.is_some());
        assert!(result.data[1].extra.is_some());
        
        let extra_country = result.data[0].extra.clone().unwrap();
        assert_eq!(extra_country.continent, Some("Europe".into()));
        assert_eq!(extra_country.world_region, Some("EMEA".into()));
        assert_eq!(extra_country.fifa, Some("GER".into()));
        assert_eq!(extra_country.iso, Some("DEU".into()));

        assert!(result.data[0].leagues.is_none());
        assert!(result.data[1].leagues.is_none());
    }

    #[test]
    fn it_returns_all_the_countries_with_details() {
        let body = fs::read_to_string(Path::new("src/support/countries/all_with.json")).expect("Fixtures:");
        let m = mock("GET", "/countries?api_token=1234&include=countries%3Afilter%28id%7C2%29")
          .with_status(200)
          .with_body(body)
          .create();

        let instance = CountryGateway::new(Gateway::new("1234".into()));
        let opts = Options::builder().include(&vec!["countries:filter(id|2)"]);
        let response = instance.all_with(opts);
        m.assert();

        assert!(response.is_ok());
        let result = response.unwrap();
        assert_eq!(result.data[0].id, 11);
        assert_eq!(result.data[0].name, "Germany".to_string());
        assert_eq!(result.data[1].name, "Northern Ireland".to_string());
        assert!(result.data[0].extra.is_some());
        assert!(result.data[1].extra.is_some());
        
        let extra_country = result.data[0].extra.clone().unwrap();
        assert_eq!(extra_country.continent, Some("Europe".into()));
        assert_eq!(extra_country.world_region, Some("EMEA".into()));
        assert_eq!(extra_country.fifa, Some("GER".into()));
        assert_eq!(extra_country.iso, Some("DEU".into()));

        assert!(result.data[0].leagues.is_some());
        let germany_leagues = result.data[0].leagues.clone().unwrap();
        assert_eq!(germany_leagues.len(), 1);
        assert_eq!(germany_leagues[0].id, 82);
        assert_eq!(germany_leagues[0].legacy_id, 4);
        assert_eq!(germany_leagues[0].country_id, 11);
        assert_eq!(germany_leagues[0].is_cup, false);
        assert_eq!(germany_leagues[0].current_season_id, 13005);
        assert_eq!(germany_leagues[0].current_round_id, Some(148761));
        assert_eq!(germany_leagues[0].current_stage_id, 7743107);
        assert_eq!(germany_leagues[0].live_standings, true);
        assert_eq!(germany_leagues[0].coverage.topscorer_goals, true);
        assert_eq!(germany_leagues[0].coverage.topscorer_assists, true);
        assert_eq!(germany_leagues[0].coverage.topscorer_cards, true);

        assert!(result.data[1].leagues.is_some());
        let north_ireland_leagues = result.data[1].leagues.clone().unwrap();
        assert_eq!(north_ireland_leagues.len(), 0);
    }

    #[test]
    fn it_returns_a_single_country() {
        let body = fs::read_to_string(Path::new("src/support/countries/find.json")).expect("Fixtures:");
        let m = mock("GET", "/countries/491?api_token=1234")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = CountryGateway::new(Gateway::new("1234".into()));
        let response = instance.find(491);
        m.assert();

        assert!(response.is_ok());
        let result = response.unwrap();

        assert_eq!(result.data.id, 491);
        assert_eq!(result.data.name, "Northern Ireland".to_string());
        assert_eq!(result.data.leagues, None);

        let extra_country = result.data.extra.clone().unwrap();
        assert_eq!(extra_country.continent, Some("Europe".into()));
        assert_eq!(extra_country.world_region, Some("EMEA".into()));
        assert_eq!(extra_country.fifa, Some("IRL".into()));
        assert_eq!(extra_country.iso, Some("IRL".into()));
        assert_eq!(extra_country.longitude, Some("-8.196102142333984".into()));
        assert_eq!(extra_country.latitude, Some("53.1827278137207".into()));
        assert!(extra_country.flag.is_some());
    }

    #[test]
    fn it_returns_a_single_country_with_details() {
        let body = fs::read_to_string(Path::new("src/support/countries/find_with.json")).expect("Fixtures:");
        let m = mock("GET", "/countries/11?api_token=1234&include=leagues%3Alimit%282%7C1%29")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = CountryGateway::new(Gateway::new("1234".into()));
        let response = instance.find_with(11, Options::builder().include(&vec!["leagues:limit(2|1)"]));
        m.assert();
        
        assert!(response.is_ok());
        let result = response.unwrap();

        assert_eq!(result.data.id, 11);
        assert_eq!(result.data.name, "Germany".to_string());
        assert!(result.data.leagues.is_some());
        
        let germany_leagues = result.data.leagues.clone().unwrap();
        assert_eq!(germany_leagues.len(), 1);
        assert_eq!(germany_leagues[0].id, 82);
        assert_eq!(germany_leagues[0].legacy_id, 4);
        assert_eq!(germany_leagues[0].country_id, 11);
        assert_eq!(germany_leagues[0].is_cup, false);
        assert_eq!(germany_leagues[0].current_season_id, 13005);
        assert_eq!(germany_leagues[0].current_round_id, Some(148761));
        assert_eq!(germany_leagues[0].current_stage_id, 7743107);
        assert_eq!(germany_leagues[0].live_standings, true);
        assert_eq!(germany_leagues[0].coverage.topscorer_goals, true);
        assert_eq!(germany_leagues[0].coverage.topscorer_assists, true);
        assert_eq!(germany_leagues[0].coverage.topscorer_cards, true);

        let extra_country = result.data.extra.clone().unwrap();
        assert_eq!(extra_country.continent, Some("Europe".into()));
        assert_eq!(extra_country.fifa, Some("GER".into()));
        assert_eq!(extra_country.iso, Some("DEU".into()));
        assert!(extra_country.flag.is_some());    
    }

    #[test]
    fn it_returns_error_if_country_does_not_exist() {
        let m = mock("GET", "/countries/1234?api_token=1234")
          .with_status(403)
          .with_body("{\"error\":{\"message\":\"yadda yadda\",\"code\":403}}")
          .create();
        
        let instance = CountryGateway::new(Gateway::new("1234".into()));
        let response = instance.find(1234);
        m.assert();
        
        assert!(response.is_err());
        let sportmonks_error = response.unwrap_err();
        assert_eq!(sportmonks_error.message(), "yadda yadda".to_string());
        assert_eq!(sportmonks_error.code(), 403);
    }

}