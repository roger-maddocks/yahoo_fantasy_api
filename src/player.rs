use crate::collision_report::CollisionReport;
use crate::Team;

enum Position {
    Center,
    LeftWing,
    RightWing,
    Defense,
    Goalie
}
enum NhlFranchise {
    FloridaPanthers(Team),
    ColoradoAvalanche(Team),
    BuffaloSabers(Team),
}
pub struct Player  {
    pub first_name: String,
    pub last_name: String,
    pub position: Vec<Position>,
    pub current_team: NhlFranchise,
    pub prioritize: bool
}

impl Player {
    pub fn new() {
        
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