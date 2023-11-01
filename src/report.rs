use std::collections::HashMap;
use crate::fantasy_week::FantasyWeek;
use crate::player::{Player, Position};
use crate::scheduled_games::Games;
use crate::team::Team;


#[derive(Default)]
pub struct Report {
    pub fantasy_weeks: Vec<FantasyWeek>,
    pub teams_playing_four_or_more: HashMap<Team, i32>,
    pub top_free_agents: Vec<Player>,
    pub game_count: HashMap<Team, i32>,
    pub front_heavy_teams: HashMap<Team, i32>, //teams with 3 games Monday - Thursday
    pub back_heavy_teams: HashMap<Team, i32>, //teams with 3 games Thursday - Sunday
    pub games_today: Games,
    pub home_teams: Vec<Team>,
    pub away_teams: Vec<Team>,
    pub index: i64
}


impl Report {
    pub fn new () -> Self {
        Default::default()
    }
    pub fn get_top_free_agent_scorers(&self, _position: Position) {
        //query Yahoo league for FA
        //Grab top X (5?) by points scored in the previous week
        //if Position not supplied, just top 5 overall scorers
    }
}
