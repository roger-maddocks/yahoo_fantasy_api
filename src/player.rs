

use crate::Team;

pub enum Position {
    Center,
    LeftWing,
    RightWing,
    Defense,
    Goalie
}
pub enum NhlFranchise {
    FloridaPanthers(Team),
    ColoradoAvalanche(Team),
    BuffaloSabers(Team),
}
pub struct Player  {
    pub first_name: String,
    pub last_name: String,
    pub position: Vec<Position>,
    pub prioritize: bool
}

impl Player {
    pub fn new() -> Player {
        Self {
            first_name: "".to_string(),
            last_name: "".to_string(),
            position: vec![],
            prioritize: false,
        }
    }

    // pub fn default() -> Self {
    //     Self
    // }

}



// pub fn collides_with(/*&self,*/ player_in_question: Player, ) -> CollisionReport {

    // collisions: CollisionReport::default();
    //
    // collisions
// }