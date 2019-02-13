extern crate sport_monks;
use std::env;
use sport_monks::gateway::Options;

fn main() {
    let api_key = env::var("API_TOKEN").expect("Set API_TOKEN env var.");
    let client = sport_monks::Client::new(api_key);
    println!("{:?}", client.contients.all());
    let options = Options::new(None, Some(vec!["countries".into()]));
    println!("{:?}", client.contients.all_with(options));
}