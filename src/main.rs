use crate::factories::weekly_data_factory::{teams_with_four_games, teams_with_three_loaded_games};
use crate::factories::yahoo_fantasy_factory::{League, YahooFantasyFactory};
use crate::models::{report::Report, player, fantasy_week::FantasyWeek, regular_season, roster, scheduled_games, team, yahoo_auth_profile };
use anyhow;
use oauth2;
use oauth2::{ErrorResponse, RevocableToken, TokenIntrospectionResponse, TokenResponse, TokenType};
use reqwest;
use serde::Serialize;
use serde_urlencoded;
use std::error::Error;
use std::ops::Add;
use crate::cachers::report_cacher::ReportCache;
use crate::models::collision_report::CollisionReport;

mod factories;
mod models;
mod builders;
mod cachers;

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
        // let report =
        let mut report_cache= ReportCache::new(async move |report_request|
            Option::from({
                async move {
                    factories::weekly_data_factory::get_loaded_schedule_report(report_request).await.unwrap()
                }
            }.await)) ;

        let report = report_cache.get_report(i).await.unwrap();
        week_index.push(i);
        weekly_reports.push(report);

        println!("WEEK {} DATA RETRIEVED", i);
    }

    let mut indexed_overloaded_report_iter = week_index.iter().zip(weekly_reports.iter()).clone();
    let mut indexed_collision_report_iter = week_index.iter().zip(weekly_reports.iter()).clone();

    for _ in 0..weekly_reports.iter().count() {
        generate_overloaded_reports(&mut indexed_overloaded_report_iter.next())
            .await;
    }

    for _ in 0..weekly_reports.iter().count() {
        get_my_collision_report(&mut indexed_collision_report_iter.next())
            .await;
    }

    Ok(())
}

//using my roster, check each position for days with more players than slots available
//3 Centers (VAN/NYR/NJD) check report for days where all 3 teams play.
//2 Centers => No collisions guaranteed

//generate roster with my 3 players
//use it to update collision report
async fn get_my_collision_report(indexed_report: &mut Option<(&u64, &Report)>) {
    let mut my_roster = YahooFantasyFactory::get_my_roster().await;
    let mut my_base_collision_report = CollisionReport::new(
        my_roster,
        Default::default(),
        Default::default(),
        0,
        Default::default()
    );

    factories::player_data_factory::get_positional_collision_report(indexed_report, my_base_collision_report);
}

async fn generate_overloaded_reports(indexed_report: &mut Option<(&u64, &Report)>) {
    teams_with_four_games(indexed_report);
    teams_with_three_loaded_games(indexed_report);
}
