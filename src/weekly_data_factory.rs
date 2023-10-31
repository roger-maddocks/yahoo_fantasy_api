use std::collections::HashMap;
use chrono::{Duration, NaiveDate};
use crate::scheduled_games::{Games};
use crate::team::Team;
use crate::fantasy_week::FantasyWeek;

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
    let mut game_count = HashMap::new();
    let mut front_heavy_teams = HashMap::new(); //teams with 3 games Monday - Thursday
    let mut back_heavy_teams = HashMap::new(); //teams with 3 games Thursday - Sunday
    let mut games_today: Games;
    let mut home_teams: Vec<Team> = vec![];
    let mut away_teams: Vec<Team> = vec![];
    let mut index: i64 = 0;
    // let mut max_count = HashMap::new();

    for _ in this_week.start.iter_days().take(7).enumerate() {

        let day = this_week.start + Duration::days(index);
        // println!("{}",day);
        games_today = get_games_for_day(&day).await;

        home_teams =  games_today.games
            .iter()
            .map(|this_game| this_game.schedule.home_team.clone())
            .collect();

        away_teams = games_today.games
            .iter()
            .map(|this_game| this_game.schedule.away_team.clone())
            .collect();

        count_games(&mut game_count, &home_teams.clone(), &away_teams.clone()).await;
        // count_games(&mut game_count, &away_teams).await;

        match index {
            x if x < 3 =>
                count_games(&mut front_heavy_teams, &home_teams, &away_teams).await,
            x if x == 3 =>
                {
                    count_games(&mut front_heavy_teams, &home_teams, &away_teams).await;
                    count_games(&mut back_heavy_teams, &home_teams, &away_teams).await;
                }
            x if x > 3 =>
                count_games(&mut back_heavy_teams, &home_teams, &away_teams).await,
            _ => panic!("Error trying to get front/back heavy schedules")
        }

        // println!("Processing day {:?}.", index+1);
        index += 1
    }

    teams_with_four_games(week, &mut game_count, &mut front_heavy_teams, &mut back_heavy_teams);
    get_overloaded_teams(&mut game_count, &mut front_heavy_teams, "front loaded");
    get_overloaded_teams(&mut game_count, &mut back_heavy_teams, "back loaded");

    println!();
    println!();
    println!();
    game_count
}

fn teams_with_four_games(week: u64, game_count: &mut HashMap<Team, i32>, front_heavy_teams: &mut HashMap<Team, i32>, back_heavy_teams: &mut HashMap<Team, i32>) {

    print_starting_block(week);
    for (key, value) in game_count.iter() {
        if *value >= 4 {
            let mut update_string = "";
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

fn print_starting_block(week: u64) {
    println!(" |--------------------------|");
    println!(" |--------- WEEK {} ---------|", week);
    println!(" |--------------------------|");
    println!();
    println!("------------------------------------");
    println!("|   Teams with 4 Games this week   |");
    println!("------------------------------------");
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

async fn count_games(game_count: &mut HashMap<Team, i32>, home_team_collection: &Vec<Team>, away_team_collection: &Vec<Team>) -> () {
    let this_clone = game_count.clone();
    let home_clone = home_team_collection.clone();
    let away_clone = away_team_collection.clone();

    update_load_count(game_count, &this_clone, home_clone);
    update_load_count(game_count, &this_clone, away_clone);
}

fn update_load_count(game_count: &mut HashMap<Team, i32>, this_clone: &HashMap<Team, i32>, home_clone: Vec<Team>) {
    for team in home_clone {
        match this_clone.get(&team) {
            Some(count) => { game_count.insert(team.clone(), count + 1); }
            None => { game_count.insert(team.clone(), 1); }
        }
    }
}
