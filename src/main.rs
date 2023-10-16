use std::collections::HashMap;
use chrono::{ Duration };
use futures::executor;
use reqwest::Error;
use crate::regular_season::{ FantasySchedule };
use crate::roster_builder::Roster;
pub mod roster_builder;
use crate::scheduled_games::{Games, Team};
mod my_sports_feed_profile;
mod scheduled_games;
mod regular_season;


#[tokio::main]
async fn main () -> Result<(), Error> {
    let game_count = executor::block_on(get_week_insights(2));

    println!("{:?}", game_count.iter());

    // let &mut game_count: HashMap<&Team, &i32>  = HashMap::new();
    // let my_roster = &Roster::new();
    // let this_weeks_games = &Games{ games: vec![] };
    // executor::block_on(get_roster(my_roster));
    // println!("{:?}", my_roster);

    Ok(())
}

async fn get_roster(my_roster: &Roster) ->  () {
    my_roster.add_player("MISTERRR SVECHNIKOVVVVVV")
}

async fn get_week_insights(week: u64) -> HashMap<Team, i32>
{
    //get specified week of season
    let this_week = FantasySchedule::get_week(&FantasySchedule {}, week);

    let mut dumb_count = HashMap::new();
    let mut game_count = HashMap::new();
    let mut games_today: Games;
    let mut home_teams: Vec<Team>;
    let mut away_teams: Vec<Team>;
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

        // println!("Home: {:#?}", games_today);


        dumb_count.insert(String::from("Anything"), 200);

        home_teams =  games_today.games
            .iter()
            .map(|this_game| this_game.schedule.home_team.clone())
            .collect();

        away_teams = games_today.games
            .iter()
            .map(|this_game| this_game.schedule.away_team.clone())
            .collect();

        println!("Home: {:?}", home_teams);
        // println!("Away: {:?}", away_teams);

        count_games(&mut game_count, &home_teams);
        count_games(&mut game_count, &away_teams);

        println!("{:?}", index);
        index += 1
    }

    println!("{:?}", dumb_count);
    game_count
}

fn count_games(game_count: &mut HashMap<Team, i32>, team_collection: &Vec<Team>) -> () {
    for team in team_collection {
        match game_count.get(&team) {
            Some(count) => { game_count.insert(team.clone(), count+1); }
            None => { game_count.insert(team.clone(), 1); }
        };
    }

}
