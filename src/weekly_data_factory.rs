use std::collections::HashMap;
use chrono::{ Duration };
use reqwest::Error;
use crate::fantasy_week::{FantasyWeek};
use crate::scheduled_games::{Games};
use crate::team::Team;
use crate::report::Report;



pub async fn get_week_reports (start_week: u64, end_week: u64) -> Report {

    let this_week= FantasyWeek::get_week_range(&FantasyWeek {
                start:  Default::default(),
                end: Default::default()
            }, start_week, end_week );

    let mut full_report = Report{ fantasy_weeks: vec![], teams_playing_four_or_more: Default::default() };

    for week in start_week .. end_week {
        full_report
            .teams_playing_four_or_more
            .push(teams_playing_four_or_more(week, &this_week[week]))
    }
    //needs to iterate each week in range

    full_report


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
    let mut games_today: Games;
    let mut home_teams: Vec<Team> = vec![];
    let mut away_teams: Vec<Team> = vec![];
    let mut index: i64 = 0;
    let mut max_count = HashMap::new();

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

        // println!("Processing day {:?}.", index+1);
        index += 1
    }

    println!("///// WEEK {} ///////", week);

    for (key, value) in game_count.iter() {
        if *value >= 4 {
            max_count.insert(key.clone(), value.clone());
            println!("Team: {} | Games: {}  ", key.abbreviation, value);
        }
        continue
    }

    println!("////////////////////");
    println!();
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
