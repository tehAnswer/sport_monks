use super::errors::SportMonksError;
use serde::de::DeserializeOwned;
use serde::Serialize;
use reqwest::{Response, Client};
use reqwest::header::{AUTHORIZATION, ACCEPT, CONTENT_TYPE, HeaderValue, HeaderMap, HeaderName};
use std::error::Error;

#[cfg(not(test))]
const BASE_URL: &str = "https://soccer.sportmonks.com/api/v2.0";
#[cfg(test)]
const BASE_URL: &str = mockito::SERVER_URL;
const ERROR_MSG: &str = "Error while parsing a SportMonks error response. Contact mantainers.";

pub struct Gateway {
    http_client: Client,
    api_key: String
}

#[derive(Default)]
pub struct Options {
    pub tz: Option<String>,
    pub include: Option<Vec<String>>
}

impl Options {
    pub fn empty() -> Options {
        Options { tz: None, include: None }
    }
    
    pub fn new(tz: Option<String>, include: Option<Vec<String>>) -> Options {
        Options { tz, include}
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
                    // println!("{}", response.text().unwrap());
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
                        Err(_) => {
                            Err(SportMonksError::new(0, ERROR_MSG.to_string())) 
                        }
                    }
                }
            },
            Err(error) => Err(SportMonksError::new(0, error.description().to_string()))
        }
    }
}