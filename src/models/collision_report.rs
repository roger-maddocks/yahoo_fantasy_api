use crate::player::Player;
use chrono::NaiveDate;
use std::collections::HashMap;
use crate::builders::roster_builder::Roster;
use crate::models::player;

enum RegularSeason {
    StartDate(NaiveDate),
    EndDate(NaiveDate),
    AllStarWeek((NaiveDate, NaiveDate)),
    OpeningWeek((NaiveDate, NaiveDate)),
    FinalWeek((NaiveDate, NaiveDate)),
}
pub struct CollisionReport {
    pub roster: Roster,
    start_week: NaiveDate,
    end_week: NaiveDate,
    season_collisions: i8,
    pub weekly_collisions: HashMap<u64, u64>, //TODO: Hashmap or Vec or other collection? Collision object > week_number, collisions
}

/// Requirements for new collision report
/// Player A : Player B :
///
impl CollisionReport {
    pub fn new(
        roster: Roster,
        start_week: NaiveDate,
        end_week: NaiveDate,
        season_collisions: i8,
        weekly_collisions: HashMap<u64, u64>,
    ) -> Self {
        Self {
            roster,
            start_week,
            end_week,
            season_collisions,
            weekly_collisions,
        }
    }

    pub fn default() -> Self {
        Self {
            roster: Default::default(),
            start_week: Default::default(),
            end_week: Default::default(),
            season_collisions: 0,
            weekly_collisions: HashMap::new()
        }
    }
}
