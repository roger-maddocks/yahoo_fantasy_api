use crate::player::NhlFranchise::ColoradoAvalanche;
use crate::Team;

#[derive(Debug)]
pub enum Position {
    Center,
    LeftWing,
    RightWing,
    Defense,
    Goalie,
}
#[derive(Debug)]
pub enum NhlFranchise {
    ColoradoAvalanche(Team),
    FloridaPanthers(Team),
    BuffaloSabers(Team),
}

impl Default for NhlFranchise { 
    fn default () -> Self {
        NhlFranchise::ColoradoAvalanche(Default::default())
    }
}

#[derive(Default, Debug)]
pub struct Player {
    pub first_name: String,
    pub last_name: String,
    pub position: Vec<Position>,
    pub prioritize: bool,
    pub team: NhlFranchise,
}

impl Player {
    pub fn new() -> Player {
        Self {
            first_name: "".to_string(),
            last_name: "".to_string(),
            position: vec![],
            prioritize: false,
            team: ColoradoAvalanche(Default::default())
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
