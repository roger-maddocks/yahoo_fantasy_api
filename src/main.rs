use std::collections::HashMap;
use chrono::{ Duration };
use reqwest::Error;
use crate::regular_season::{ FantasySchedule };
pub mod roster_builder;
use crate::scheduled_games::{Games};
use crate::team::Team;
mod my_sports_feed_profile;
mod scheduled_games;
mod regular_season;
mod team;
mod weekly_data_factory;
mod report;
mod fantasy_week;


#[tokio::main]
async fn main () -> Result<(), Error> {
    for i in 2..5 {
        let this_week = FantasySchedule::get_week_range(&FantasySchedule {}, i, i );
        weekly_data_factory::teams_playing_four_or_more(i, &this_week).await;
    }

    Ok(())
}
