use reqwest::Error;
use reqwest::{Client, ClientBuilder};
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
mod yahoo_auth_profile;

#[tokio::main]
async fn main () -> Result<(), Error> {
    for i in 5 ..= 5  {
        let this_week = FantasyWeek::new(i, i);
        weekly_data_factory::get_loaded_schedule_report(i, &this_week).await;
    }

    Ok(())
}
//
// pub struct YahooConnector {
//     client: Client,
//     url: &'static str,
//     search_url: &'static str,
// }
//
// #[derive(Default)]
// pub struct YahooConnectorBuilder {
//     inner: ClientBuilder,
// }
//
// impl YahooConnector {
//
//     pub fn new() -> YahooConnector {
//         YahooConnector {
//             client: Client::default(),
//             url: env![""],
//             search_url: env![""],
//         }
//     }
// }











