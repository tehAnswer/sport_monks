extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
#[cfg(test)]
extern crate mockito;

pub mod ops;
pub mod models;
pub mod errors;
pub mod wrapper;
pub mod gateway;

use gateway::{Gateway, Options};


pub struct Client {
    pub contients: ops::ContinentGateway,
}

impl Client {
    pub fn new<S: Into<String>>(api_key: S) -> Client {
        Client {
            contients: ops::ContinentGateway::new(Gateway::new(api_key.into())),
        }
    }
}
