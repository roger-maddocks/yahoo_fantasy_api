use crate::models::fantasy_week::FantasyWeek;
use crate::models::report::Report;
use crate::scheduled_games::Games;
use crate::team::Team;
use chrono::{Duration, NaiveDate};
use std::collections::HashMap;

///
///
/// # Arguments
///
/// * `week`:
///
/// returns: HashMap<Team, i32, RandomState>
///
/// # Examples
///
/// ```
/// EOF after 20 consecutive weeks
/// WEEK 26 Final week of season
/// ```
pub async fn get_loaded_schedule_report(this_week: &FantasyWeek) -> Option<Report> {
    let mut report = Report::default();

    for days in this_week.start.iter_days().take(7).enumerate() {
        report.daily_games.push(get_games_for_day(&days.1).await);

        let report_iter = report
                .daily_games
                .iter()
                .nth(report.index as usize)
                .unwrap()
                .games
                .iter();

        report.home_teams = report_iter
            .to_owned()
            .map(|each_game| each_game.schedule.home_team.to_owned())
            .collect();

        report.away_teams = report_iter
            .to_owned()
            .map(|each_game| each_game.schedule.away_team.to_owned())
            .collect();

        register_games(
            &mut report.game_count,
            &report.home_teams.to_owned(),
            &report.away_teams.to_owned(),
        )
        .await;

        match report.index {
            x if x < 3 => {
                register_games(
                    &mut report.front_heavy_teams,
                    &report.home_teams,
                    &report.away_teams,
                )
                .await
            }
            x if x == 3 => {
                register_games(
                    &mut report.front_heavy_teams,
                    &report.home_teams,
                    &report.away_teams,
                )
                .await;
                register_games(
                    &mut report.back_heavy_teams,
                    &report.home_teams,
                    &report.away_teams,
                )
                .await;
            }
            x if x > 3 => {
                register_games(
                    &mut report.back_heavy_teams,
                    &report.home_teams,
                    &report.away_teams,
                )
                .await
            }
            _ => panic!("Error while trying to determine front/back heavy schedules"),
        }

        report.index += 1
    }

    // println!("{:?}", report);
    Some(report)
}


async fn get_games_for_day(date: &NaiveDate) -> Games {
    let daily_url: String =
        "https://api.mysportsfeeds.com/v2.1/pull/nhl/2023-regular/games.json?date=".to_owned();


    let games_today = reqwest::Client::new()
        .get(daily_url + &date.format("%Y%m%d").to_string())
        .basic_auth(env!("MSF_API_KEY"), Some(env!("MSF_PASSWORD")))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    // USE THIS TO SEE FULL JSON RETURNED
    // let games_test = reqwest::Client::new()
    //     .get(daily_url.clone() + &date.format("%Y%m%d").to_string())
    //     .basic_auth(env!("MSF_API_KEY"), Some(env!("MSF_PASSWORD")))
    //     .send()
    //     .await
    //     .unwrap()
    //     .text()
    //     .await
    //     .unwrap();
    // println!("{:?}", games_test);

    games_today
}

pub(crate) fn teams_with_four_games(indexed_report: &mut Option<(&u64, &Report)>) {

    let mut this_indexed_report= indexed_report.unwrap();
    let mut report = this_indexed_report.1;

    print_starting_block(this_indexed_report.0.to_owned());


    for (key, value) in report.game_count.iter() {
        if *value >= 4 {
            // let mut update_string = "";
            // max_count.insert(key.clone(), value.clone());
            if report.front_heavy_teams.get_key_value(key) == Some((&key, &3)) {
                println!("| Team: {} | front heavy schedule |", key.abbreviation);
            } else if report.back_heavy_teams.get_key_value(key) == Some((&key, &3)) {
                println!("| Team: {} | back heavy schedule  |", key.abbreviation);
            } else {
                println!("| Team: {} | distributed schedule |", key.abbreviation);
            }
        }
        continue;
    }
    println!("------------------------------------");
    println!();
}

pub(crate) fn teams_with_three_loaded_games(indexed_report: &mut Option<(&u64, &Report)>) {

    let mut this_indexed_report= indexed_report.unwrap();
    let mut report = this_indexed_report.1;

    get_loaded_teams(
        &mut report.game_count.clone(),
        &mut report.front_heavy_teams.clone(),
        "front loaded",
    );

    get_loaded_teams(
        &mut report.game_count.to_owned(),
        &mut report.back_heavy_teams.to_owned(),
        "back loaded",
    );
}

pub(crate) fn get_loaded_teams(
    game_count: &mut HashMap<Team, i32>,
    loaded_teams: &mut HashMap<Team, i32>,
    description: &str,
) {
    let mut index: i32 = 0;

    format_team_workload_separator(description);

    for (key, value) in loaded_teams.iter() {
        if *value >= 3 {
            if game_count.get_key_value(key) != Some((&key, &4)) {
                print!("|");
                print!("Team: {} | {} lineup|", key.abbreviation, description);
                print!(" ");
                index += 1;
            }
        }
        continue;
    }
    if index < 1 {
        println!("No teams {} this week", description);
    }
    println!();
}

async fn register_games(
    game_count: &mut HashMap<Team, i32>,
    home_teams: &Vec<Team>,
    away_teams: &Vec<Team>,
) -> () {
    update_load_count(game_count, home_teams);
    update_load_count(game_count, away_teams);
}

fn update_load_count(game_count: &mut HashMap<Team, i32>, team_collection: &Vec<Team>) {
    for team in team_collection {
        match game_count.get(&team) {
            Some(count) => {
                game_count.insert(team.to_owned(), count + 1);
            }
            None => {
                game_count.insert(team.to_owned(), 1);
            }
        }
    }
}

fn format_team_workload_separator(description: &str) {

    format_based_on_description(description);
    println!(
        "| Teams playing less than four games with {} lineup |",
        description
    );
    format_based_on_description(description);
}

fn format_based_on_description(description: &str) {
    match description {
        x if x.contains("front") => {
            println!("---------------------------------------------------------------")
        }
        x if x.contains("back") => {
            println!("--------------------------------------------------------------")
        }
        _ => panic!("Error formatting front/back description"),
    }
}

fn print_starting_block(week: u64) {
    println!("|--------------------------|");
    println!("|--------- WEEK {} ---------|", week);
    println!("|--------------------------|");
    println!();
    println!("------------------------------------");
    println!("|   Teams with 4 Games this week   |");
    println!("------------------------------------");
}
