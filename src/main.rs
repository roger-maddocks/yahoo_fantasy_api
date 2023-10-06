use reqwest::Error;
use crate::regular_season::{FantasySchedule, FantasyWeek};
use crate::scheduled_games::Games;
mod my_sports_feed_profile;
mod scheduled_games;
mod regular_season;


#[tokio::main]
async fn main () -> Result<(), Error> {

    let this_week = get_week_insights(2);

    println!("---------- FIRST DAY ----------");
    // println!("{:#?}", this_week.iter_mut());

    Ok(())
}


async fn get_week_insights(week: u64) -> Games
{
    println!("--You're not here either--");
    let mut daily_url: String = "https://api.mysportsfeeds.com/v2.1/pull/nhl/2023-regular/games.json?date=".to_owned();
    let our_week = FantasySchedule::get_start_week(&FantasySchedule {});
    let first_day = our_week.start.format("%Y%m%d").to_string();

    if week == 1 {
        let our_week = FantasySchedule::get_start_week(&FantasySchedule {});
        let first_day = our_week.start.format("%Y%m%d").to_string();
        let first_games: Games = reqwest::Client::new()
            .get(daily_url + &first_day)
            .basic_auth(env!("MY_SPORTS_FEEDS_API_KEY"), Some(env!("MY_SPORTS_FEEDS_PASSWORD")))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        println!("--You're not here--");
        return first_games;
    }

    let games: Games = reqwest::Client::new()
        .get(daily_url + &first_day)
        .basic_auth(env!("MY_SPORTS_FEEDS_API_KEY"), Some(env!("MY_SPORTS_FEEDS_PASSWORD")))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    println!("--You're here--");
    println!("{:#?}", games);

    return games;
}



// use std::borrow::Borrow;
//datime crate (chrono?)
//iter is a borrow of the thing
//into_iter is an owned version


// ----------------------------------------------------------------------------
// use reqwest::RequestBuilder;
// use serde::{Deserialize, Serialize};
// use serde_json;
// use crate::my_sports_feed_profile::ProfileBuilder;
//
// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all="camelCase")]
// struct Games {
//     games: Vec<Game>
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all="camelCase")]
// struct Game {
//     schedule: Schedule,
//     score: Score
// }
//
//
// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all="camelCase")]
// struct Score {
//     awayScoreTotal: i32,
//     homeScoreTotal: i32
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all="camelCase")]
// struct Schedule {
//     awayTeam: Team,
//     homeTeam: Team,
//     startTime: String
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all="camelCase")]
// struct Team {
//     abbreviation: String,
// }
//

/////////////MAIN
// let mut nhl_game_response = client
//     .get("https://api.mysportsfeeds.com/v2.1/pull/nhl/2023-pre/games.json?date=20230925")
//     // .basic_auth(profile.borrow())
//     .basic_auth("163db9fd-bdbe-4bd4-a4b8-15324f", Some("MYSPORTSFEEDS"))
//     .send()
//     .await?
//     .text()
//     .await?;

// let profile = ProfileBuilder::new().build();
// let key: MySportsFeedProfile = ProfileBuilder::new().name(String::from("163db9fd-bdbe-4bd4-a4b8-15324f"), "MYSPORTSFEEDS".to_string()).build();
// println!("{:#?}", game);
//
// println!("{:?}", nhl_game_response);
// // let json: serde_json::Value = serde_json::from_str(&nhl_game_response.into_ok());
//

//
// let mut allGamesAvailable = (Schedule, Score) = reqwest::Client::new()
//     .get("https://api.mysportsfeeds.com/v2.1/pull/nhl/2023-pre/games.json?date=20230925")
//     .basic_auth("163db9fd-bdbe-4bd4-a4b8-15324f", Some("MYSPORTSFEEDS"))
//     .send()
//     .await?
//     .json()
//     .await?;
// // let profile = ProfileBuilder::new().build();
// // let key: MySportsFeedProfile = ProfileBuilder::new().name(String::from("163db9fd-bdbe-4bd4-a4b8-15324f"), "MYSPORTSFEEDS".to_string()).build();
// println!("{:#?}", allGamesAvailable);

