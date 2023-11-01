// use anyhow;
use oauth2::*;
use reqwest;
use url::Url;

use serde_json::ser::State;
// use yahoo_fantasy_api::yahoo_connector;
use crate::fantasy_week::FantasyWeek;
use crate::team::Team;
pub mod roster_builder;
mod my_sports_feed_profile;
mod scheduled_games;
mod regular_season;
mod team;
mod player;
mod weekly_data_factory;
mod report;
mod fantasy_week;
mod collision_report;

#[tokio::main]
async fn main () -> Result<(), reqwest::Error> {


    let result = reqwest::blocking::get("https://api.spotify.com/v1/search").await;

    for i in 4 ..= 4  {
        let this_week = FantasyWeek::new(i, i);
        weekly_data_factory::get_loaded_schedule_report(i, &this_week).await;
    }

    Ok(())
}
pub struct ReceivedCode {
    pub code: AuthorizationCode,
    pub state: State,
}

//main chunk
// let builder = YahooExtensionsBuilder::
// tokio::task::spawn_blocking(|| {
//     let this_client = local_yahoo_connector();
// }).await.expect("Thread panicked");

// #[tokio::main]
// pub async fn local_yahoo_connector() -> Result<(), Error> {
//
//     let client =
//         BasicClient::new(
//             ClientId::new(env!["YAHOO_CLIENT_ID"].to_string()),
//             Some(ClientSecret::new(env!["YAHOO_CLIENT_SECRET"].to_string())),
//             AuthUrl::new(env!["YAHOO_AUTH_ENDPOINT"].to_string())?,
//             Some(TokenUrl::new(env!["YAHOO_TOKEN_URL"].to_string())?))
//         ;
//     let token_result = client
//         .exchange_client_credentials()
//         .add_scope(Scope::new("read".to_string()))
//         .request(http_client)?;
//     println!("token result: {:?}", token_result);
//
//
//     Ok(())
// }