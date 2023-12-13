use crate::player::Player;
use chrono::NaiveDate;
use std::collections::HashMap;

enum RegularSeason {
    StartDate(NaiveDate),
    EndDate(NaiveDate),
    AllStarWeek((NaiveDate, NaiveDate)),
    OpeningWeek((NaiveDate, NaiveDate)),
    FinalWeek((NaiveDate, NaiveDate)),
}
pub struct CollisionReport {
    player_ayy: Option<Player>,
    player_bee: Option<Player>,
    player_see: Option<Player>,
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
        player_ayy: Option<Player>,
        player_bee: Option<Player>,
        player_see: Option<Player>,
        start_week: NaiveDate,
        end_week: NaiveDate,
        season_collisions: i8,
        weekly_collisions: HashMap<i8, i8>,
    ) -> Self {
        Self {
            player_ayy,
            player_bee,
            player_see,
            start_week,
            end_week,
            season_collisions,
            weekly_collisions,
        }
    }
    pub fn default() -> Self {
        Self {
            player_ayy: Some(Player::new()),
            player_bee: Some(Player::new()),
            player_see: Some(Player::new()),
            start_week: Default::default(),
            end_week: Default::default(),
            season_collisions: 0,
            weekly_collisions: Default::default(),
        }
    }
}
