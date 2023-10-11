use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

#[derive(Debug, Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Games {
    pub games: Vec<Game>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub schedule: Schedule,
    pub score: Score,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub away_score_total: Option<i32>,
    pub home_score_total: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub away_team: Team,
    pub home_team: Team,
    pub start_time: String,
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub abbreviation: String,
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
    pub fn get_weekly_games() -> Result<Games, reqwest::Error> {

        Ok(Games { games: vec![] })
    }

    pub fn get_daily_games(&self, day: NaiveDate) -> Result<Games, reqwest::Error> {

        Ok(Games { games: vec![] })
    }

}

impl Iterator for GameIntoIterator {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl PartialEq for Team {
    fn eq(&self, other:&Team) -> bool {
        self.abbreviation == other.abbreviation
    }
}