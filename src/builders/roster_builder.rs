use crate::models::player::Position::{C, D, LW, RW};
use crate::models::player::{Player, Position};
use std::cell::RefCell;

#[derive(Debug, Default, Clone)]
pub struct Roster {
    pub players: Vec<Player>,
}

impl Roster {
    pub fn new() -> Roster {
        Roster { players: vec![] }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player.to_owned());
    }

    //based on your players, which weeks/days will you be FORCED to sit one player
    pub fn get_collision_report(&self) -> () {}

    pub fn get_player_by_position(&self, positions: Vec<Position>) -> Vec<Player> {
        let mut players = vec![];
        let position_iter = positions.iter();

        position_iter.for_each(|x| {
            for player in &self.players {
                if player.position.contains(x) {
                    players.push(player.clone())
                }
            }
        });
        players
    }
}

//
// impl RosterBuild for Roster {
//     fn add_player(&self, player: &str) {
//         self.players.borrow_mut().push(String::from(player));
//     }
// }
