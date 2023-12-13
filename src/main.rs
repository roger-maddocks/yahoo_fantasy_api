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
use crate::report::Report;
use crate::weekly_data_factory::{teams_with_three_loaded_games, teams_with_four_games};

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
    let mut weekly_reports: Vec<Report> =  vec![];
    let mut week_index= vec![];

    let result = fantasy_factory
        .yahoo_client
        .get_access_token()
        .await
        .expect("Main access token error!");

    println!("{:#?}", result);

    for i in 9..=11 {
        week_index.push(i);
        let this_week = FantasyWeek::new(i, i);
        let report = weekly_data_factory::get_loaded_schedule_report(i, &this_week).await; // println!("{:#?}", report);

        weekly_reports.push(report);
        println!("WEEK {} DATA RETRIEVED", i);
    }

    let mut indexed_report_iter = week_index.iter().zip(weekly_reports.iter());

    for _ in 0 .. weekly_reports.iter().count() {
        generate_overloaded_reports(&mut indexed_report_iter.next());
    }

    Ok(())
}

fn generate_overloaded_reports(indexed_report: &mut Option<(&u64, &Report)>) {
    teams_with_four_games(indexed_report); //this_indexed_report.0.to_owned(), this_indexed_report.1);
    teams_with_three_loaded_games(indexed_report);
}

