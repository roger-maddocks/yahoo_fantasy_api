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
/// WEEK 28 Final week of season
/// ```
pub async fn teams_playing_four_or_more(week: u64, this_week: &FantasyWeek) -> HashMap<Team, i32>
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
        println!("{}",day);
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

    println!("///// WEEK {} ///////", week);

    for (key, value) in game_count.iter() {
        if *value >= 4 {
            let mut update_string = "";
            // max_count.insert(key.clone(), value.clone());
            if front_heavy_teams.get_key_value(key) == Some((&key, &3)) {
                println!("Team: {} | Games: {} | Front heavy lineup ", key.abbreviation, value);
            }
            else if back_heavy_teams.get_key_value(key) == Some((&key, &3)) {
                println!("Team: {} | Games: {} | Back heavy lineup ", key.abbreviation, value);
            }
            else {
                println!("Team: {} | Games: {}  ", key.abbreviation, value);
            }
        }
        continue
    }

    println!("//////////////////////////////");
    println!("Teams ONLY front-loaded //////");
    for (key, value) in front_heavy_teams.iter() {
        if *value >= 3 {
            if game_count.get_key_value(key) != Some((&key, &4)) {
                println!("Team: {} | Games: {} | Front heavy ONLY lineup ", key.abbreviation, value);
            }
        }
        continue
    }

    println!("//////////////////////////////");
    println!("Teams ONLY back-loaded ///////");
    for (key, value) in back_heavy_teams.iter() {
        if *value >= 3 {
            if game_count.get_key_value(key) != Some((&key, &4)) {
                println!("Team: {} | Games: {} | Back heavy ONLY lineup ", key.abbreviation, value);
            }
        }
        continue
    }
    println!("////////////////////");
    println!();
    game_count
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
