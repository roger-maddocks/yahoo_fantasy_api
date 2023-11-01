use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::team::Team;

#[derive(Debug, Clone, Default, Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Games {
    pub games: Vec<Game>
}

#[derive(Debug, Clone, Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub schedule: Schedule,
    pub score: Score,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub away_score_total: Option<i32>,
    pub home_score_total: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub away_team: Team,
    pub home_team: Team,
    pub start_time: String,
}

impl IntoIterator for Game {
    type Item = ();
    type IntoIter = GameIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

pub struct GameIntoIterator {
    game: Game,
    index: usize,
}

impl Game {
}

impl Games {
    pub fn new() -> Games {
        Games {
            games: vec![]
        }
    }
    pub fn get_weekly_games() -> Result<Games, reqwest::Error> {

        Ok(Games { games: vec![] })
    }

    pub fn get_daily_games(&self, day: NaiveDate) -> Result<Games, reqwest::Error> {

        if day.leap_year() { }

        Ok(Games { games: vec![] })
    }

}

impl Iterator for GameIntoIterator {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}