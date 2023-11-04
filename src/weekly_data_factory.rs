use std::collections::HashMap;
use chrono::{Duration, NaiveDate};
use crate::scheduled_games::{Games};
use crate::team::Team;
use crate::fantasy_week::FantasyWeek;
use crate::report::Report;

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
pub async fn get_loaded_schedule_report(week: u64, this_week: &FantasyWeek) -> HashMap<Team, i32>
{
    //move all into report struct
    let mut report = Report::new();

    for _ in this_week.start.iter_days().take(7).enumerate() {

        let day = this_week.start + Duration::days(report.index);
        // println!("{}",day);
        report.games_today = get_games_for_day(&day).await;

        report.home_teams =  report.games_today.games
            .iter()
            .map(|this_game| this_game.schedule.home_team.clone())
            .collect();

        report.away_teams = report.games_today.games
            .iter()
            .map(|this_game| this_game.schedule.away_team.clone())
            .collect();

        count_games(&mut report.game_count, &report.home_teams.clone(), &report.away_teams.clone()).await;
        // count_games(&mut game_count, &away_teams).await;

        match report.index {
            x if x < 3 =>
                count_games(&mut report.front_heavy_teams, &report.home_teams, &report.away_teams).await,
            x if x == 3 =>
                {
                    count_games(&mut report.front_heavy_teams, &report.home_teams, &report.away_teams).await;
                    count_games(&mut report.back_heavy_teams, &report.home_teams, &report.away_teams).await;
                }
            x if x > 3 =>
                count_games(&mut report.back_heavy_teams, &report.home_teams, &report.away_teams).await,
            _ => panic!("Error trying to get front/back heavy schedules")
        }

        // println!("Processing day {:?}.", index+1);
        report.index += 1
    }

    teams_with_four_games(week, &mut report.game_count, &mut report.front_heavy_teams, &mut report.back_heavy_teams);
    get_overloaded_teams(&mut report.game_count, &mut report.front_heavy_teams, "front loaded");
    get_overloaded_teams(&mut report.game_count, &mut report.back_heavy_teams, "back loaded");

    println!();
    println!();
    println!();
    report.teams_playing_four_or_more
}

pub async fn get_max_remaining_games_report() {
    //return report of teams with most remaining games from today until end of week
}

pub async fn get_following_week_early_games_report() {
    //return report of teams with frontloaded schedule next week
    //return back loaded schedule if before thursday
    //return raw game count if thursday or later
}
pub async fn get_doubled_up_weekend_report() {
    //return report of teams with two weekend games
}

async fn get_games_for_day(date: &NaiveDate) -> Games {
    let daily_url: String = "https://api.mysportsfeeds.com/v2.1/pull/nhl/2023-regular/games.json?date=".to_owned();

    let games_today = reqwest::Client::new()
        .get(daily_url + &*date.format("%Y%m%d").to_string())
        .basic_auth(env!("MY_SPORTS_FEEDS_API_KEY"), Some(env!("MY_SPORTS_FEEDS_PASSWORD")))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    games_today
}

fn teams_with_four_games(week: u64, game_count: &mut HashMap<Team, i32>, front_heavy_teams: &mut HashMap<Team, i32>, back_heavy_teams: &mut HashMap<Team, i32>) {

    print_starting_block(week);

    for (key, value) in game_count.iter() {
        if *value >= 4 {
            // let mut update_string = "";
            // max_count.insert(key.clone(), value.clone());
            if front_heavy_teams.get_key_value(key) == Some((&key, &3)) {
                println!("| Team: {} | front heavy schedule |", key.abbreviation);
            } else if back_heavy_teams.get_key_value(key) == Some((&key, &3)) {
                println!("| Team: {} | back heavy schedule  |", key.abbreviation);
            } else {
                println!("| Team: {} | distributed schedule |", key.abbreviation);
            }
        }
        continue
    }
    println!("------------------------------------");
    println!();
}


fn get_overloaded_teams(game_count: &mut HashMap<Team, i32>, loaded_teams: &mut HashMap<Team, i32>, description: &str) {
    let mut index: i32  = 0;

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
        continue
    }
    if index < 1 { println!("No teams {} this week", description);}
    println!();
}

async fn count_games(game_count: &mut HashMap<Team, i32>, home_team_collection: &Vec<Team>, away_team_collection: &Vec<Team>) -> () {
    update_load_count(game_count, home_team_collection);
    update_load_count(game_count, away_team_collection);
}

fn update_load_count(game_count: &mut HashMap<Team, i32>, team_collection: &Vec<Team>) {
    for team in team_collection {
        match game_count.get(&team) {
            Some(count) => { game_count.insert(team.clone(), count + 1); }
            None => { game_count.insert(team.clone(), 1); }
        }
    }
}

fn format_team_workload_separator(description: &str) {
    match description {
        x if x.contains("front") => println!("---------------------------------------------------------------"),
        x if x.contains("back") => println!("--------------------------------------------------------------"),
        _ => panic!("Error formatting front/back loaded weeks")
    }

    println!("| Teams playing less than four games with {} lineup |", description);

    match description {
        x if x.contains("front") => println!("---------------------------------------------------------------"),
        x if x.contains("back") => println!("--------------------------------------------------------------"),
        _ => panic!("Error formatting front/back loaded weeks")
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
