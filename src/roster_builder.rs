use std::cell::RefCell;
use std::thread::Builder;

#[derive(Debug)]
pub struct Roster {
    players: RefCell<Vec<String>>
}

impl Roster {

    pub fn new() -> Roster {
        Roster {
            players: RefCell::new(vec![])
        }
    }

    pub fn add_player(&self, player: &str) {
        self.players.borrow_mut().push(String::from(player));
    }

    //based on your players, which weeks/days will you be FORCED to sit one player
    pub fn get_collision_report(&self) ->  () {
    }
}

//
// impl RosterBuild for Roster {
//     fn add_player(&self, player: &str) {
//         self.players.borrow_mut().push(String::from(player));
//     }
// }

struct Player {
    first_name: String,
    last_name: String,
    team: String
}



