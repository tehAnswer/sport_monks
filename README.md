# SportMonks

SportMonks is a company specialized in providing stats, insights and historical results of different sports. This crate implements support for their [Football API](https://www.sportmonks.com/docs/football/2.0/prologue/a/introduction/94).

# Functionalities

> _*NOTE*: For a better understanding of this client or SportMonks' API capabilities, refer to its official documentation linked above._

Enlisted below, you can find a short list of common requests among many others that you can make with this adapter:

- Request statistics of a finished or in-live game.
- Request information about football competitions around the globe.
- Request information about teams and their players.

These scenarios are exemplified down below.

# Install

To install the crate, add the following line to your Cargo.toml

```
sport_monks = "0.1.2"
```

# Usage

## Request statistics about a game

```rust
extern crate sport_monks;

use sport_monks::*;

fn main() {
    let client = Client::new("YOUR_API_TOKEN");
    let options = Options::builder().include(&["localTeam", "visitorTeam", "stats"]);
    let query = client.fixtures.find_with(11414789, options);
    match query {
        Ok(response) => {
            let game = response.data;
            let home_team = game.local_team.unwrap().name; 
            let away_team = game.visitor_team.unwrap().name;
            println!("{} {} - {} {}",
              home_team,
              game.scores.localteam_score,
              away_team,
              game.scores.visitorteam_score);
            
            let stats = game.stats.unwrap();
            println!("POS: {}% - {}%", stats[0].possessiontime, stats[1].possessiontime);
            println!("SHOTS: {} - {}", stats[0].shots.ongoal, stats[1].shots.ongoal);
            println!("FOULS: {} - {}", stats[0].fouls, stats[1].fouls);
            println!("FREE KICKS: {} - {}", stats[0].free_kick, stats[1].free_kick);
            println!("ATTACKS: {} - {}", stats[0].attacks.dangerous_attacks, stats[1].attacks.dangerous_attacks);

        },
        Err(sport_monks) => println!("{:?}", sport_monks),
    }
}
```

Should print in the console:

```
Real Madrid 1 - Ajax 4
POS: 57% - 43%
SHOTS: 8 - 8
FOULS: 12 - 11
FREE KICKS: 18 - 13
ATTACKS: 78 - 41
```

## Request information about football competitions around the globe

````rust
extern crate sport_monks;

use sport_monks::*;

fn main() {
    let client = Client::new("YOUR_API_TOKEN");
    let query = client.leagues.all();
    match query {
        Ok(response) => {
            let leagues = response.data;
            for league in leagues {
                let options = Options::builder().include(&vec!["standings.team"]);
                match client.standings.find_with(league.current_season_id, options) {
                    Ok(second_response) => {
                        let stadings = &second_response.data[0].standings;
                        println!("{}: {} {}", league.name, stadings[0].team_name, stadings[0].points)
                    },
                    Err(sport_monks) => println!("{:?}", sport_monks),
                }
            }
        },
        Err(sport_monks) => println!("{:?}", sport_monks),
    }
}
```

The snippet should print the leader boards of all the football competitions included in your plan.


## Request information about teams and their players

```rust
extern crate sport_monks;

use sport_monks::*;

fn main() {
    let client = Client::new("API_TOKEN");
    let options = Options::builder().include(&["squad.player"]);
    let query = client.teams.find_with(7980, options);
    match query {
        Ok(response) => {
            let team = response.data;
            for player in team.squad.unwrap() {
                let player = player.player.unwrap();
                println!("{} ({}) plays at {}", player.fullname.unwrap(), player.nationality.unwrap(), team.name)
            }
        },
        Err(sport_monks) => println!("{:?}", sport_monks),
    }
}
```

The snippet above should print the name and nationality of Atletico's players.
 








