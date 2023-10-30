use std::collections::HashMap;
use crate::fantasy_week::FantasyWeek;
use crate::player::{Player, Position};
use crate::team::Team;


pub struct Report {
    pub fantasy_weeks: Vec<FantasyWeek>,
    pub teams_playing_four_or_more: Vec<HashMap<Team, i32>>,
    pub top_free_agents: Vec<Player>
}


impl Report {
    pub fn get_top_free_agent_scorers(&self, _position: Position) {
        //query Yahoo league for FA
        //Grab top X (5?) by points scored in the previous week
        //if Position not supplied, just top 5 overall scorers
    }

}

