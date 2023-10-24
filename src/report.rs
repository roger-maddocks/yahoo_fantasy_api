use std::collections::HashMap;
use crate::fantasy_week::FantasyWeek;
use crate::team::Team;

pub struct Report {
    pub fantasy_weeks: Vec<FantasyWeek>,
    pub teams_playing_four_or_more: Vec<HashMap<Team, i32>>

}
