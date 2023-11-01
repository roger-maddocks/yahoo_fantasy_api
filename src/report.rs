use std::collections::HashMap;
use crate::fantasy_week::FantasyWeek;
use crate::player::{Player, Position};
use crate::scheduled_games::Games;
use crate::team::Team;


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
    pub fn get_top_free_agent_scorers(&self, _position: Position) {
        //query Yahoo league for FA
        //Grab top X (5?) by points scored in the previous week
        //if Position not supplied, just top 5 overall scorers
    }

    pub fn new(fantasy_weeks: Vec<FantasyWeek>, teams_playing_four_or_more: HashMap<Team, i32>, top_free_agents: Vec<Player>, game_count: HashMap<Team, i32>, front_heavy_teams: HashMap<Team, i32>, back_heavy_teams: HashMap<Team, i32>, games_today: Games, home_teams: Vec<Team>, away_teams: Vec<Team>, index: i64) -> Self {
        Self { fantasy_weeks, teams_playing_four_or_more, top_free_agents, game_count, front_heavy_teams, back_heavy_teams, games_today, home_teams, away_teams, index }
    }
}

