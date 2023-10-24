use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::team::Team;

#[derive(Debug, Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Games {
    pub games: Vec<Game>
}

#[derive(Debug, Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub schedule: Schedule,
    pub score: Score,
}

impl Clone for Game {
    fn clone(&self) -> Self {
        Game{
            schedule: self.schedule.clone(),
            score: self.score.clone()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub away_score_total: Option<i32>,
    pub home_score_total: Option<i32>,
}

// impl Clone for Score {
//     fn clone(&self) -> Self {
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub away_team: Team,
    pub home_team: Team,
    pub start_time: String,
}

impl Clone for Schedule {
    fn clone(&self) -> Self {
        Schedule {
            away_team: self.away_team.clone(),
            home_team: self.home_team.clone(),
            start_time: self.start_time.clone(),

        }
    }
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