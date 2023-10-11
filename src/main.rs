use std::collections::HashMap;
use chrono::{Days, Duration, NaiveDate, Weekday};
use futures::executor;
use reqwest::Error;
use crate::regular_season::{FantasySchedule, FantasyWeek};
use crate::scheduled_games::{Games, Team};
mod my_sports_feed_profile;
mod scheduled_games;
mod regular_season;


#[tokio::main]
async fn main () -> Result<(), Error> {

    executor::block_on(get_week_insights(2));

    Ok(())
}


async fn get_week_insights(week: u64) -> ()
{
    //
    let this_week = FantasySchedule::get_week(&FantasySchedule {}, week);

    let mut game_count: HashMap<&Team, i32>;
    let mut index: i64 = 0;

    for i in this_week.start.iter_days().take(7).enumerate() {
        let mut daily_url: String = "https://api.mysportsfeeds.com/v2.1/pull/nhl/2023-regular/games.json?date=".to_owned();
        let first_day = this_week.start + Duration::days(index);

        let this_week: Games = reqwest::Client::new()
            .get(daily_url + &first_day.format("%Y%m%d").to_string())
            .basic_auth(env!("MY_SPORTS_FEEDS_API_KEY"), Some(env!("MY_SPORTS_FEEDS_PASSWORD")))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let mut home_teams: Vec<_> = this_week.games
            .iter()
            .map(|this_game| &this_game.schedule.home_team)
            .collect();

        let mut away_teams: Vec<_> = this_week.games
            .iter()
            .map(|this_game| &this_game.schedule.away_team)
            .collect();

        println!("{:?}", home_teams);
        println!("{:?}", away_teams);


        let mut game_count = HashMap::new();
        for team in home_teams {
            match game_count.get(team) {
                Some(count) => { game_count.insert(team, count+1); }
                None => { game_count.insert(team, 1); }
            };
        }

        for team in away_teams {
            match game_count.get(team) {
                Some(count) => { game_count.insert(team, count+1); }
                None => { game_count.insert(team, 1); }
            };
        }
        println!("{:?}", game_count);
        println!("{:?}", index);
        index += 1
    }
}
