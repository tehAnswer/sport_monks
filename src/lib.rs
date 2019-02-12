extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
#[cfg(test)]
extern crate mockito;

pub mod ops;

pub struct Client {
    api_key: String
}

impl Client {
    pub fn new<S: Into<String>>(api_key: S) -> Client {
        Client { api_key: api_key.into() }
    }
}
