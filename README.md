# SportMonks

SportMonks is a company specialized in providing stats, insights and historical results of different sports. This crate implements support for their [Football API](https://www.sportmonks.com/docs/football/2.0/prologue/a/introduction/94).

> **NOTE**: This crate is still in its beta version. Hence, it isn't production ready yet.

# Usage

Add the following line to your Cargo.toml

```
sport_monks = "0.1"
```

Then, import the library into your project by adding the following line:

```
extern crate sport_monks;
```

Eventually, create a `sport_monks::Client` instance from your private token.

```
use std::env;
use sport_monks::gateway::Options;

fn main() {
    let api_key = env::var("API_TOKEN").expect("Set API_TOKEN env var.");
    let client = sport_monks::Client::new(api_key);
    // ...
}
```

# Examples

TBD
 








