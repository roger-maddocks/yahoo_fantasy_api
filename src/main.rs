use std::collections::HashMap;
use chrono::{ Duration };
use reqwest::Error;
use crate::fantasy_week::{ FantasyWeek };
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
        //should return FantasyWeek with all data
        let report = weekly_data_factory::get_week_reports(i, i );

    }

    Ok(())
}
