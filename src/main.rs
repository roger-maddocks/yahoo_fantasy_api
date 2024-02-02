use crate::factories::weekly_data_factory::{teams_with_four_games, teams_with_three_loaded_games};
use crate::factories::yahoo_fantasy_factory::{League, YahooFantasyFactory};
use crate::models::{report::Report, player, scheduled_games, team, yahoo_auth_profile };
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
use crate::models::collision_report::CollisionReport;
use crate::models::fantasy_week::FantasyWeek;

mod factories;
mod models;
mod builders;
mod helpers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    loop {
        // let _= fantasy_factory.await.get_free_agents().await;
        let mut fantasy_factory = YahooFantasyFactory::new_factory(League::Nhl).shared();

        let user_input = get_user_command()
            .to_lowercase()
            .trim()
            .to_owned();

        match user_input {
            x if exit_program(&x) => {
                println!("L8r sk8r");
                break
            }
            x if x.to_lowercase().trim() == "fa" => {
                fantasy_factory
                    .await
                    .get_free_agents()
                    .await
                    .unwrap()
            }
            x if x.to_lowercase().trim() == "wr" => {
                get_overloaded_report().await;
            }
            x if x.to_lowercase().trim() == "s" => {
                not_implemented(&x)
            }
            x => {
                println!("{:?} is not a supported command!", x)
            }
        }
    }

    Ok(())
}

async fn get_overloaded_report() {
    let mut week_index = vec![];
    let mut weekly_reports: Vec<Report> = vec![];
    let report = factories::weekly_data_factory::get_loaded_schedule_report(&FantasyWeek::new(get_user_report_bounds())).await;

    week_index.push(1);
    weekly_reports.push(report);

    let mut report_base = week_index
        .iter()
        .zip(weekly_reports.iter());
    let mut indexed_overloaded_report_iter = report_base.clone();
    // let mut indexed_collision_report_iter = report_base.clone();

    for _ in 0..weekly_reports.len() {
        generate_overloaded_reports(&mut indexed_overloaded_report_iter.next())
            .await;
    }

    // for _ in 0..weekly_reports.iter().count() {
    // get_my_collision_report(&mut indexed_collision_report_iter.next())
    //     .await;
    // }

    // let report = factories::weekly_data_factory::get_loaded_schedule_report(&FantasyWeek::new(get_user_report_bounds())).await;
    // let _ = generate_overloaded_reports(fantasy_factory.await.yahoo_client)
    // not_implemented(&x)
}

fn get_user_report_bounds() -> u64 {
    println!("Enter which week (1 - 26) you would like the report for.");

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Issue reading User input!");

    input.trim().to_lowercase().parse().expect("User did not enter numeric value!")
}

fn exit_program(input: &str) -> bool {
    match input {
        x if x == "q" => {
            true
        }
        x if x == "quit" => {
            true
        }
        _ => {
            false
        }
    }
}

fn not_implemented(x: &str) {
    println!("Whoops, I misguided you. The {:?} functionality is not supported yet!", x);
}

fn get_user_command() -> String {
    print_program_options();

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Issue reading User input!");
    input
        .to_lowercase()
        .trim()
        .parse()
        .unwrap()
}

fn print_program_options() {
    println!("Enter \"S\" for initial setup.");
    println!("Enter \"FA\" for Free Agency operations.");
    println!("Enter \"WR\" to view Week Report from specific week.");
    println!("Enter \"Q\" to quit.");
}

async fn generate_overloaded_reports(indexed_report: &mut Option<(&u64, &Report)>) {
    teams_with_four_games(indexed_report).await;
    teams_with_three_loaded_games(indexed_report).await;
}

///using my roster, check each position for days with more players than slots available
///3 Centers (VAN/NYR/NJD) check report for days where all 3 teams play.
///2 Centers => No collisions guaranteed
async fn get_my_collision_report(indexed_report: &mut Option<(&u64, &Report)>, factory: &YahooFantasyFactory) {
    // let mut my_roster = factory.get_my_roster().await;
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

