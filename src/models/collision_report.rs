use crate::player::Player;
use chrono::NaiveDate;
use std::collections::HashMap;
use crate::builders::roster_builder::Roster;

enum RegularSeason {
    StartDate(NaiveDate),
    EndDate(NaiveDate),
    AllStarWeek((NaiveDate, NaiveDate)),
    OpeningWeek((NaiveDate, NaiveDate)),
    FinalWeek((NaiveDate, NaiveDate)),
}
pub struct CollisionReport {
    roster: Roster,
    start_week: NaiveDate,
    end_week: NaiveDate,
    season_collisions: i8,
    weekly_collisions: HashMap<i8, i8>, //TODO: Hashmap or Vec or other collection? Collision object > week_number, collisions
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
        weekly_collisions: HashMap<i8, i8>,
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
            weekly_collisions: Default::default(),
        }
    }
}
