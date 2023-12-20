use crate::yahoo_auth_profile::YahooResponses;
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
use chrono::NaiveDate;
use futures::TryFutureExt;
use crate::factories::yahoo_fantasy_factory;
use crate::models::collision_report::CollisionReport;

mod factories;
mod models;
mod builders;
mod helpers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    yahoo_auth_profile::YahooAuthClient::get_redirect_url_for_auth_code();

    let mut fantasy_factory = YahooFantasyFactory::new_factory(League::Nhl);

    println!("before token update {:?}", fantasy_factory.yahoo_client.access_token);

    fantasy_factory.yahoo_client.refresh_access_token().await;

    println!("after token update {:?}", fantasy_factory.yahoo_client.access_token);

    let my_free_agents = fantasy_factory.get_free_agents().await;
    println!("League Free Agents: {:?}", my_free_agents);
    //
    // let my_roster = YahooFantasyFactory::get_my_roster(&fantasy_factory).await;
    // println!("My Roster : {:?}", my_roster);
    //
    // let my_league_resource = fantasy_factory.get_league_resource();
    // println!("League Resource: {:?}", my_league_resource.await);

    let mut week_index = vec![];
    let mut weekly_reports: Vec<Report> = vec![];
    for i in 14..=14 {
        let report = factories::weekly_data_factory::get_loaded_schedule_report(&FantasyWeek::new(i,i)).await;

        week_index.push(i);
        weekly_reports.push(report);

        println!("WEEK {} DATA RETRIEVED", i);
    }

    let mut report_base = week_index.iter().zip(weekly_reports.iter());
    let mut indexed_overloaded_report_iter = report_base.clone();
    let mut indexed_collision_report_iter = report_base.clone();

    for _ in 0..weekly_reports.len() {
        generate_overloaded_reports(&mut indexed_overloaded_report_iter.next())
            .await;
    }


    println!();
    println!();
    for _ in 0..weekly_reports.iter().count() {
        get_my_collision_report(&mut indexed_collision_report_iter.next())
            .await;
    }

    Ok(())
}
// let result = fantasy_factory
//     .yahoo_client
//     .get_access_token_json()
//     .await;

//using my roster, check each position for days with more players than slots available
//3 Centers (VAN/NYR/NJD) check report for days where all 3 teams play.
//2 Centers => No collisions guaranteed

async fn get_my_collision_report(indexed_report: &mut Option<(&u64, &Report)>) {
    // let mut my_roster = YahooFantasyFactory::get_my_roster().await;
    // let mut my_base_collision_report = CollisionReport::new(
    //     my_roster,
    //     Default::default(),
    //     Default::default(),
    //     0,
    //     Default::default()
    // );
    //
    // factories::player_data_factory::get_positional_collision_report(indexed_report, &mut my_base_collision_report);
}

async fn generate_overloaded_reports(indexed_report: &mut Option<(&u64, &Report)>) {
    teams_with_four_games(indexed_report).await;
    teams_with_three_loaded_games(indexed_report).await;
}
