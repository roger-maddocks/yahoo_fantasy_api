// use crate::models;
// use crate::models::report::Report;
// use crate::models::team::Team;
use crate::factories::weekly_data_factory::{teams_with_four_games, teams_with_three_loaded_games};
use crate::factories::yahoo_fantasy_factory::{League, YahooFantasyFactory};
use anyhow;
use oauth2;
use oauth2::{ErrorResponse, RevocableToken, TokenIntrospectionResponse, TokenResponse, TokenType};
use reqwest;
use serde::Serialize;
use serde_urlencoded;
use std::error::Error;
use std::ops::Add;
// use crate::models::report::Report;
use crate::models::my_sports_feed_profile;
use crate::models::{report::Report, player, fantasy_week::FantasyWeek, regular_season, roster, scheduled_games, team, yahoo_auth_profile };
// mod models::{}collision_report;
// mod player;
// mod regular_season;
// mod report;
// pub mod roster_builder;
//
// mod scheduled_games;
// mod team;
// // mod factories::weekly_data_factory;
// mod yahoo_auth_profile;
mod factories;
mod models;
mod builders;
// mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    models::yahoo_auth_profile::YahooConnection::get_redirect_url_for_auth_code();

    let fantasy_factory = YahooFantasyFactory::new_factory(League::Nhl);
    let mut week_index = vec![];
    let mut weekly_reports: Vec<Report> = vec![];

    let result = fantasy_factory
        .yahoo_client
        .get_access_token()
        .await
        .expect("Main access token error!");

    println!("{:#?}", result);

    for i in 9..=11 {
        let report = factories::weekly_data_factory::get_loaded_schedule_report(&FantasyWeek::new(i, i)).await; // println!("{:#?}", report);

        week_index.push(i);
        weekly_reports.push(report);

        println!("WEEK {} DATA RETRIEVED", i);
    }

    let mut indexed_report_iter = week_index.iter().zip(weekly_reports.iter()).clone();

    for _ in 0..weekly_reports.iter().count() {
        generate_overloaded_reports(&mut indexed_report_iter.next())
            .await;
    }

    Ok(())
}

async fn generate_overloaded_reports(indexed_report: &mut Option<(&u64, &Report)>) {
    teams_with_four_games(indexed_report); //this_indexed_report.0.to_owned(), this_indexed_report.1);
    teams_with_three_loaded_games(indexed_report);
}
