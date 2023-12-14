use crate::models::collision_report::CollisionReport;
use crate::models::player::Position;
use crate::models::report::Report;
use crate::models::team::Team;
use crate::player::Player;

pub fn get_positional_collision_report(indexed_report: &mut Option<(&u64, &Report)>, collision_base: CollisionReport) {

    let mut this_indexed_report= indexed_report.unwrap();
    let mut report = this_indexed_report.1;
    // let collisions = vec![];
    // u8, crate::models::player::Position, Vec< crate::models::player::Player >
    for (key) in report.daily_games.iter() {
        let daily_games = key.games.iter();
        let collision_count = 0;

        for each_game in daily_games {
            println!("{:?}", each_game)
        }

        //     // let mut update_string = "";
        //     // max_count.insert(key.clone(), value.clone());
        //     if report.front_heavy_teams.get_key_value(key) == Some((&key, &3)) {
        //         println!("| Team: {} | front heavy schedule |", key.abbreviation);
        //     } else if report.back_heavy_teams.get_key_value(key) == Some((&key, &3)) {
        //         println!("| Team: {} | back heavy schedule  |", key.abbreviation);
        //     } else {
        //         println!("| Team: {} | distributed schedule |", key.abbreviation);
        //     }
        // }
        // continue;
    }
}