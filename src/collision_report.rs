use crate::team::Team;
use chrono::NaiveDate;

struct CollisionReport {
    first_team: Team,
    second_team: Team,
    start_week: NaiveDate,
    end_week: NaiveDate
}