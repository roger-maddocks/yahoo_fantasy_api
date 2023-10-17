use std::collections::HashMap;
use crate::team::Team;
use chrono::NaiveDate;

pub struct CollisionReport {
    pub first_team: Team,
    pub second_team: Team,
    pub start_week: NaiveDate,
    pub end_week: NaiveDate,
    pub collision_total: i32,
    pub collisions_by_week: HashMap<i32, i32>,
    pub yearly_report: bool
}