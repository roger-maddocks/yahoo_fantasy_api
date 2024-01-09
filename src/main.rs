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
    let mut user_input= "".to_string();

    loop {
        user_input = get_user_command();

        if exit_program(&user_input) {
            println!("L8r sk8r");
            break
        }

        perform_user_operation(user_input).await;


        println!("input: {:?}", user_input);

        // match user_input {
        //     _ =>  ()
        // }
    }

    Ok(())
}

async fn perform_user_operation(input: String) ->  Result<(), Box<dyn Error>> {
    let mut fantasy_factory = YahooFantasyFactory::new_factory(League::Nhl).shared();
    let mut local_factory = fantasy_factory.clone();
    let _= local_factory.await.get_free_agents().await;

    match input.to_lowercase().trim() {
        x if x == "setup" => {}
        x if x.to_lowercase() == "fa" => local_factory.await.get_free_agents(),
        _ => {
            false
        }
    }
    Ok(())
}

fn exit_program(input: &String) -> bool {

    println!("User Entered: {:?}", &input);

    match input.to_lowercase().trim() {
        x if x == "q" => {
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
}

fn print_program_options() {
    println!("Enter \"S\" for initial setup.");
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

