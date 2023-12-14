use std::cell::RefCell;
use crate::models::player::Player;

#[derive(Debug, Default, Clone)]
pub struct Roster {
    players: Vec<Player>,
}

impl Roster {
    pub fn new() -> Roster {
        Roster {
            players: vec![],
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player.to_owned());
    }

    //based on your players, which weeks/days will you be FORCED to sit one player
    pub fn get_collision_report(&self) -> () {}
}

//
// impl RosterBuild for Roster {
//     fn add_player(&self, player: &str) {
//         self.players.borrow_mut().push(String::from(player));
//     }
// }
