use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Games {
    games: Vec<Game>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Game {
    schedule: Schedule,
    score: Score,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Score {
    away_score_total: Option<i32>,
    home_score_total: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Schedule {
    away_team: Team,
    home_team: Team,
    start_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    abbreviation: String,
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

impl Iterator for GameIntoIterator {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
