use super::super::errors::SportMonksError;
use super::super::models::{Continent,Wrapper};
use super::super::gateway::{Gateway,Options};


pub struct ContinentGateway {
    gateway: Gateway
}

impl ContinentGateway {
    pub fn new(gateway: Gateway) -> ContinentGateway {
        ContinentGateway { gateway }
    }

    pub fn all(&self) -> Result<Wrapper<Vec<Continent>>, SportMonksError> {
        self.gateway.get("/continents", Options::empty())
    }
    
    pub fn all_with(&self, options: Options) -> Result<Wrapper<Vec<Continent>>, SportMonksError> {
        self.gateway.get("/continents", options)
    }
    
    pub fn find(&self, id: i64) -> Result<Wrapper<Continent>, SportMonksError> {
        let path = format!("/continents/{}", id);
        self.gateway.get(&path, Options::empty())
    }

    pub fn find_with(&self, id: i64, options: Options) -> Result<Wrapper<Continent>, SportMonksError> {
        let path = format!("/continents/{}", id);
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
    fn it_returns_all_the_continents() {
        let body = fs::read_to_string(Path::new("src/support/continents/all.json")).expect("Fixtures:");
        let m = mock("GET", "/continents?api_token=1234")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = ContinentGateway::new(Gateway::new("1234".into()));
        let response = instance.all();
        m.assert();

        assert!(response.is_ok());
        let result = response.unwrap();
        assert_eq!(result.data[0].id, 1);
        assert_eq!(result.data[0].name, "Europe".to_string());
        assert_eq!(result.data[0].countries, None);
    }

    #[test]
    fn it_returns_all_the_continents_with_details() {
        let body = fs::read_to_string(Path::new("src/support/continents/all_with.json")).expect("Fixtures:");
        let m = mock("GET", "/continents?api_token=1234&include=leagues")
          .with_status(200)
          .with_body(body)
          .create();

        let instance = ContinentGateway::new(Gateway::new("1234".into()));
        let opts = Options::builder().include(&vec!["leagues"]);
        let response = instance.all_with(opts);
        
        m.assert();
        assert!(response.is_ok());
        let result = response.unwrap();
        assert_eq!(result.data[0].id, 1);
        assert_eq!(result.data[0].name, "Europe".to_string());
        assert!(result.data[0].countries.is_some());
        // COUNTRIES
        let countries = result.data[0].countries.clone().unwrap();
        assert_eq!(countries.len(), 1);
        assert_eq!(countries[0].id, 2);
        // POLSKA STRONK!
        assert_eq!(countries[0].name, "Poland");
        assert!(countries[0].extra.is_some());

        let extra_country = countries[0].extra.clone().unwrap();
        assert_eq!(extra_country.continent, Some("Europe".into()));
        assert_eq!(extra_country.world_region, Some("EMEA".into()));
        assert_eq!(extra_country.fifa, Some("POL".into()));
        assert_eq!(extra_country.iso, Some("POL".into()));
        assert_eq!(extra_country.longitude, Some("19.37775993347168".into()));
        assert_eq!(extra_country.latitude, Some("52.147850036621094".into()));
        assert!(extra_country.flag.is_some());
    }

    #[test]
    fn it_returns_a_single_continent() {
        let body = fs::read_to_string(Path::new("src/support/continents/find.json")).expect("Fixtures:");
        let m = mock("GET", "/continents/1?api_token=1234")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = ContinentGateway::new(Gateway::new("1234".into()));
        let response = instance.find(1);
        m.assert();

        assert!(response.is_ok());
        let result = response.unwrap();

        assert_eq!(result.data.id, 1);
        assert_eq!(result.data.name, "Europe".to_string());
        assert_eq!(result.data.countries, None);
    }

    #[test]
    fn it_returns_a_single_continent_with_details() {
        let body = fs::read_to_string(Path::new("src/support/continents/find_with.json")).expect("Fixtures:");
        let m = mock("GET", "/continents/1?api_token=1234&include=leagues")
          .with_status(200)
          .with_body(body)
          .create();

        
        let instance = ContinentGateway::new(Gateway::new("1234".into()));
        let response = instance.find_with(1, Options::builder().include(&vec!["leagues"]));
        m.assert();

        assert!(response.is_ok());
        let result = response.unwrap();
        assert_eq!(result.data.id, 1);
        assert_eq!(result.data.name, "Europe".to_string());
        assert!(result.data.countries.is_some());
        // COUNTRIES
        let countries = result.data.countries.clone().unwrap();
        assert_eq!(countries.len(), 2);
        assert_eq!(countries[0].name, "Poland");
        assert_eq!(countries[1].name, "Germany");
    }

    #[test]
    fn it_returns_error_if_continent_does_not_exist() {
        let m = mock("GET", "/continents/1234?api_token=1234")
          .with_status(403)
          .with_body("{\"error\":{\"message\":\"yadda yadda\",\"code\":403}}")
          .create();
        
        let instance = ContinentGateway::new(Gateway::new("1234".into()));
        let response = instance.find(1234);
        m.assert();
        
        assert!(response.is_err());
        let sportmonks_error = response.unwrap_err();
        assert_eq!(sportmonks_error.message(), "yadda yadda".to_string());
        assert_eq!(sportmonks_error.code(), 403);
    }

}