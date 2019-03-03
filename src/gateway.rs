use super::errors::SportMonksError;
use serde::de::DeserializeOwned;
use reqwest::{Response, Client};
use std::error::Error;

#[cfg(not(test))]
const BASE_URL: &str = "https://soccer.sportmonks.com/api/v2.0";
#[cfg(test)]
const BASE_URL: &str = mockito::SERVER_URL;

pub struct Gateway {
    http_client: Client,
    api_key: String
}

#[derive(Default)]
pub struct Options {
    pub tz: Option<String>,
    pub include: Option<Vec<String>>,
    pub page: Option<i64>
}

impl Options {
    pub fn empty() -> Options {
        Options::default()
    }

    pub fn builder() -> Options {
        Options::default()
    }
    
    pub fn page(mut self, page: i64) -> Options {
        self.page = Some(page);
        self
    }

    pub fn include(mut self, include: &[&str]) -> Options {
        let normalized_include = include.into_iter().map(ToString::to_string).collect();
        self.include = Some(normalized_include);
        self
    }
}

impl Gateway {
    pub fn new(api_key: String) -> Gateway {
        Gateway { http_client: Client::new(), api_key: api_key }
    }

    pub fn get<U: DeserializeOwned>(&self, path: &str, options: Options) -> Result<U, SportMonksError> {
        let url = format!("{}{}", BASE_URL, path);
        let query_string = self.prepare_options(options);
        let result = self.http_client
            .get(url.as_str())
            .query(&query_string)
            .send();
        self.handle_response(result)
    }

    fn prepare_options(&self, options: Options) -> Vec<(String, String)> {
        let mut query_string: Vec<(String, String)> = vec![("api_token".to_string(), self.api_key.to_string())];
        if options.tz.is_some() {
            query_string.push(("tz".to_string(), options.tz.unwrap()));
        }

        if options.include.is_some() {
            let joined_includes = options.include.unwrap().join(",");
            query_string.push(("include".to_string(), joined_includes));
        }
        query_string
    }

    fn handle_response<U: DeserializeOwned>(&self, result: Result<Response, reqwest::Error>) -> Result<U,  SportMonksError> {
        match result {
            Ok(mut response) => {
                if response.status().is_success() {
                    match response.json::<U>() {
                        Ok(final_response) => Ok(final_response),
                        Err(error) => {
                            Err(SportMonksError::new(0, error.to_string()))
                        }
                    }
                } else {
                    let parsed_result = response.json::<SportMonksError>();
                    match parsed_result {
                        Ok(sportmonks_error) => { Err(sportmonks_error) }, 
                        Err(x) => {    
                            Err(SportMonksError::new(0, x.to_string())) 
                        }
                    }
                }
            },
            Err(error) => Err(SportMonksError::new(0, error.description().to_string()))
        }
    }
}