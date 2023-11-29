use crate::fantasy_week::FantasyWeek;
use crate::player::NhlFranchise;
use crate::team::Team;
use crate::yahoo_fantasy_factory::{League, YahooFantasyFactory};
use anyhow;
use oauth2;
use oauth2::{ErrorResponse, RevocableToken, TokenIntrospectionResponse, TokenResponse, TokenType};
use reqwest;
use serde::Serialize;
use serde_urlencoded;
use std::error::Error;
use std::ops::Add;

mod collision_report;
mod fantasy_week;
mod my_sports_feed_profile;
mod player;
mod regular_season;
mod report;
pub mod roster_builder;
mod scheduled_games;
mod team;
mod weekly_data_factory;
mod yahoo_auth_profile;
mod yahoo_fantasy_factory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    yahoo_auth_profile::YahooConnection::get_redirect_url_for_auth_code();

    let fantasy_factory = YahooFantasyFactory::new_factory(League::Nhl);

    let result = fantasy_factory
        .yahoo_client
        .get_access_token()
        .await
        .expect("Main access token error!");
    // let result = fantasy_factory
    //     .yahoo_client
    //     .get_access_token()
    //     .await
    //     .expect("Error getting access_token!");
    //
    // println!("{:#?}", result);

    println!("{:#?}", result);

    for i in 8..=8 {
        let this_week = FantasyWeek::new(i, i);
        weekly_data_factory::get_loaded_schedule_report(i, &this_week).await;
    }

    // fantasy_factory.get_league_resource().await;
    // let game_form = fantasy_factory
    // .get_league_resource()
    // .await
    // .expect("Error asking Yahoo!");

    for i in 8..=8 {
        let this_week = FantasyWeek::new(i, i);

        weekly_data_factory::get_loaded_schedule_report(i, &this_week).await;
    }

    Ok(())
}
