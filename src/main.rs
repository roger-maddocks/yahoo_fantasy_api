use std::collections::HashMap;
use chrono::{ Duration };
use reqwest::Error;
use crate::regular_season::{ FantasySchedule };
pub mod roster_builder;
use crate::scheduled_games::{Games};
use crate::team::Team;
mod my_sports_feed_profile;
mod scheduled_games;
mod regular_season;
mod team;



#[tokio::main]
async fn main () -> Result<(), Error> {

    let game_count = get_week_insights(2).await;
    let mut max_count = HashMap::new();

    for (key, value) in game_count.iter() {
        println!("Team: {} ", key.abbreviation);
        println!("    Number of Games: {} ", value);
        if *value == 4 {
            max_count.insert(key.clone(), value.clone());
        }
        continue
    }

    println!("{:?}", game_count);

    println!("{:?}", max_count);

    Ok(())
}

async fn get_week_insights(week: u64) -> HashMap<Team, i32>
{
    //get specified week of season
    let this_week = FantasySchedule::get_week(&FantasySchedule {}, week);

    let mut game_count = HashMap::new();
    let mut games_today: Games;
    let mut home_teams: Vec<Team> = vec![];
    let mut away_teams: Vec<Team> = vec![];
    let mut index: i64 = 0;

    for _ in this_week.start.iter_days().take(7).enumerate() {
        let daily_url: String = "https://api.mysportsfeeds.com/v2.1/pull/nhl/2023-regular/games.json?date=".to_owned();
        let first_day = this_week.start + Duration::days(index);

        games_today = reqwest::Client::new()
            .get(daily_url + &first_day.format("%Y%m%d").to_string())
            .basic_auth(env!("MY_SPORTS_FEEDS_API_KEY"), Some(env!("MY_SPORTS_FEEDS_PASSWORD")))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        home_teams =  games_today.games
            .iter()
            .map(|this_game| this_game.schedule.home_team.clone())
            .collect();

        away_teams = games_today.games
            .iter()
            .map(|this_game| this_game.schedule.away_team.clone())
            .collect();

        count_games(&mut game_count, &home_teams).await;
        count_games(&mut game_count, &away_teams).await;

        println!("Processing day {:?}.", index+1);
        index += 1
    }

    game_count
}

async fn count_games(game_count: &mut HashMap<Team, i32>, team_collection: &Vec<Team>) -> () {
    for team in team_collection {
        match game_count.get(&team) {
            Some(count) => { game_count.insert(team.clone(), count+1); }
            None => { game_count.insert(team.clone(), 1); }
        };
    }
}

// fn parse_daily_schedule(teams: &mut Vec<Team>, games_today: &Games) -> () {
//     teams = games_today.games
//         .iter()
//         .map(|this_game| this_game.schedule.away_team.clone())
//         .collect();
// }
