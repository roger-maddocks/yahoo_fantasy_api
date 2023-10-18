use std::collections::HashMap;
use chrono::NaiveDate;
use crate::player::Player;

enum RegularSeason {
    StartDate(NaiveDate),
    EndDate(NaiveDate),
    AllStarWeek((NaiveDate, NaiveDate)),
    OpeningWeek((NaiveDate, NaiveDate)),
    FinalWeek((NaiveDate, NaiveDate)),
}
pub struct CollisionReport {
    player_ayy: Player,
    player_bee: Player,
    start_week: NaiveDate,
    end_week: NaiveDate,
    season_collisions: i8,
    weekly_collisions: HashMap(i8, i8) //TODO: Hashmap or Vec or other collection? Collision object > week_number, collisions
}

impl CollisionReport {
    pub fn new(player_ayy: Player,
               player_bee: Player,
               start_week: NaiveDate,
               end_week: NaiveDate,
               season_collisions: i8,
               weekly_collisions: HashMap(i8, i8)) -> Self {
        Self { player_ayy, player_bee, start_week, end_week, season_collisions, weekly_collisions }
    }
    pub fn default() -> Self {
        Self {
            player_ayy: Player::new(),
            player_bee: Player::new(),
            start_week: Default::default(),
            end_week: Default::default(),
            season_collisions: 0,
            weekly_collisions: Default::default(),
        }
    }
}




