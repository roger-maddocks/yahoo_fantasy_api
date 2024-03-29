use crate::helpers::{general_helpers, visual_helpers};
use crate::models::collision_report::CollisionReport;
use crate::models::player::Position;
use crate::models::player::Position::C;
use crate::models::report::Report;
use crate::models::team::Team;
use crate::player::Player;

pub fn get_positional_collision_report(
    indexed_report: &mut Option<(&u64, &Report)>,
    mut collision_base: &mut CollisionReport,
) {
    let mut this_indexed_report = indexed_report.unwrap();
    let mut report = this_indexed_report.1;
    let mut team_ids = vec![];
    let mut weekday = 0;
    let mut position = C;

    for (player) in collision_base
        .roster
        .get_player_by_position(vec![position.clone()])
    {
        team_ids.push(player.team.msf_id)
    }

    visual_helpers::format_collision_report_block(&mut this_indexed_report);

    for (key) in report.daily_games.iter() {
        let daily_games = key.games.iter();
        let mut collision_count = 0;

        for each_game in daily_games {
            if team_ids.contains(&each_game.schedule.home_team.msf_id) {
                collision_count += 1;
                // println!("home: {:?}, collision_count: {:?}, weekday: {:?}", each_game.schedule.home_team, collision_count, weekday)
            }

            if team_ids.contains(&each_game.schedule.away_team.msf_id) {
                collision_count += 1;
                // println!("away: {:?}, collision_count: {:?}, weekday: {:?}", each_game.schedule.away_team, collision_count, weekday)
            }

            if collision_count > 2 {
                println!(
                    "Inserting Collision! day: {:#?}, week: {:#?}, position: {:#?}",
                    general_helpers::get_day_from_number(&weekday),
                    this_indexed_report.0.clone(),
                    position
                );
                collision_base
                    .weekly_collisions
                    .insert(this_indexed_report.0.clone(), weekday);
                break;
            }
        }
        if collision_count < 3 {
            // println!("Day {:?} of week {:?} had no collisions", weekday, this_indexed_report.0.clone());
        }
        weekday += 1;
    }
}
