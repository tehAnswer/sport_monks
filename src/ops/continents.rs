use super::super::errors::SportMonksError;
use super::super::wrapper::Wrapper;
use super::super::models::Continent;
use super::super::gateway::{Gateway, Options};

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
    
    pub fn find(&self, id: u64) -> Result<Continent, SportMonksError> {
        let path = format!("/continents/{}", id);
        self.gateway.get(&path, Options::empty())
    }

    pub fn find_with(&self, id: u64, options: Options) -> Result<Continent, SportMonksError> {
        let path = format!("/continents/{}", id);
        self.gateway.get(&path, options)
    }

}