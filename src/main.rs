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
use std::io::stdin;
use futures::{FutureExt, TryFutureExt};
mod factories;
mod models;
mod builders;
mod helpers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // yahoo_auth_profile::YahooAuthClient::get_redirect_url_for_auth_code();
    let mut fantasy_factory = YahooFantasyFactory::new_factory(League::Nhl).shared();
    let mut user_input= String::new();
    // user_input = get_user_command();

    while valid_request(&get_user_command()) {
        user_input = get_user_command();
        let mut local_factory = fantasy_factory.clone();
        let _= local_factory.await.get_free_agents().await;

        println!("a{:?}", user_input);

        // match user_input {
        //     // x if x.to_lowercase() == "fa" => local_factory.await.get_free_agents(),
        //     _ =>  ()
        // }
    }
    Ok(())
}

fn valid_request(input: &String) -> bool {
    println!("b{:?}", input);
    match input {
        x if x.to_lowercase() == "q" => false,
        x if x.is_empty() => false,
        _ => true
    }
}

fn get_user_command() -> String {
    println!("Enter \"Q\" to quit, or simply press enter with a blank line.");

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Issue reading User input!");
    input
}

async fn generate_overloaded_reports(indexed_report: &mut Option<(&u64, &Report)>) {
    teams_with_four_games(indexed_report).await;
    teams_with_three_loaded_games(indexed_report).await;
}

///using my roster, check each position for days with more players than slots available
///3 Centers (VAN/NYR/NJD) check report for days where all 3 teams play.
///2 Centers => No collisions guaranteed
async fn get_my_collision_report(indexed_report: &mut Option<(&u64, &Report)>, factory: &YahooFantasyFactory) {
    // let mut my_roster = factory.get_my_roster().await.unwrap();
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


// let my_free_agents = fantasy_factory.await.get_free_agents().await;
// println!("League Free Agents: {:?}", my_free_agents);


// let my_league_resource = fantasy_factory.await.get_league_resource();
// println!("League Resource: {:?}", my_league_resource.await);

// let mut week_index = vec![];
// let mut weekly_reports: Vec<Report> = vec![];
//
// for i in 14..=14 {
//     let report = factories::weekly_data_factory::get_loaded_schedule_report(&FantasyWeek::new(i,i)).await;
//
//     week_index.push(i);
//     weekly_reports.push(report);
//
//     println!("WEEK {} DATA RETRIEVED", i);
// }
//
// let mut report_base = week_index.iter().zip(weekly_reports.iter());
// let mut indexed_overloaded_report_iter = report_base.clone();
// let mut indexed_collision_report_iter = report_base.clone();
//
// for _ in 0..weekly_reports.len() {
//     generate_overloaded_reports(&mut indexed_overloaded_report_iter.next())
//         .await;
//
// }
//
// for _ in 0..weekly_reports.iter().count() {
//     // get_my_collision_report(&mut indexed_collision_report_iter.next())
//     //     .await;
// }

