use serde::{Deserialize, Serialize};
use crate::models::player::Position::Center;
use crate::player::NhlFranchise::ColoradoAvalanche;
use crate::models::team::Team;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub first_name: String,
    pub last_name: String,
    pub position: Vec<Position>,
    pub prioritize: bool,
    pub franchise: NhlFranchise,
    pub team: Team,
    pub yahoo_team_key: String
}

impl Player {
    pub fn new(
        first_name: String,
        last_name: String,
        position: Vec<Position>,
        prioritize: bool,
        franchise: NhlFranchise,
        team: Team,
        yahoo_team_key: String
    ) -> Player {
        Self {
            first_name,
            last_name,
            position,
            prioritize,
            franchise,
            team,
            yahoo_team_key
        }
    }
    pub fn default() -> Player {
        Self {
            first_name: "".to_string(),
            last_name: "".to_string(),
            position: vec![Center],
            prioritize: false,
            franchise: ColoradoAvalanche,
            team: Default::default(),
            yahoo_team_key: "".to_string()
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum Position {
    #[default]
    Center,
    LeftWing,
    RightWing,
    Defense,
    Goalie,
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
    fn default () -> Self {
        ColoradoAvalanche
    }
}

impl PartialEq for NhlFranchise {
    fn eq(&self, other: &NhlFranchise) -> bool {
        self == other
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
