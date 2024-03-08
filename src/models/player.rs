use crate::models::player::Position::C;
use crate::models::team::Team;
use crate::player::NhlFranchise::ColoradoAvalanche;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub name: Option<Name>,
    pub position: Vec<Position>,
    pub prioritize: bool,
    pub franchise: NhlFranchise,
    pub team: Team,
    pub yahoo_team_key: String,
}

impl Player {
    pub fn new(
        first_name: String,
        last_name: String,
        position: Vec<Position>,
        prioritize: bool,
        franchise: NhlFranchise,
        team: Team,
        yahoo_team_key: String,
    ) -> Player {
        Self {
            name: Option::from(Name::new(first_name, last_name)),
            position,
            prioritize,
            franchise,
            team,
            yahoo_team_key,
        }
    }
    pub fn default() -> Player {
        Self {
            name: Default::default(),
            position: vec![C],
            prioritize: false,
            franchise: ColoradoAvalanche,
            team: Default::default(),
            yahoo_team_key: "".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum Position {
    #[default]
    C,
    LW,
    RW,
    D,
    G,
}
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq)]
pub enum NhlFranchise {
    ColoradoAvalanche,
    FloridaPanthers,
    BuffaloSabers,
    NewYorkRangers,
    NewJerseyDevils,
    VancouverCanucks,
}

impl Default for NhlFranchise {
    fn default() -> Self {
        ColoradoAvalanche
    }
}

impl PartialEq for NhlFranchise {
    fn eq(&self, other: &NhlFranchise) -> bool {
        self == other
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Name {
    #[serde(rename = "full")]
    pub full_name: String,
    #[serde(rename = "first")]
    pub first_name: String,
    #[serde(rename = "last")]
    pub last_name: String,
    pub ascii_first: String,
    pub ascii_last: String,
}
impl Name {
    pub fn new(first: String, last: String) -> Name {
        Name {
            full_name: first.clone() + &last.clone(),
            first_name: first,
            last_name: last,
            ascii_first: "".to_string(),
            ascii_last: "".to_string(),
        }
    }
}

// pub fn default() -> Self {
//     Self
// }

// pub fn collides_with(/*&self,*/ player_in_question: Player, ) -> CollisionReport {

// collisions: CollisionReport::default();
//
// collisions
// }
