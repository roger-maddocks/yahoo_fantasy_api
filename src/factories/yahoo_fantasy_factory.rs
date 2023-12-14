use crate::yahoo_auth_profile::YahooConnection;
use reqwest::Error;
use std::fmt;
use std::fmt::Formatter;
use crate::builders::roster_builder::Roster;
use crate::models::player::{NhlFranchise, Player, Position};
use crate::models::player::NhlFranchise::VancouverCanucks;
use crate::models::team::Team;

#[derive(Debug)]
pub enum League {
    Nhl,
    Nba,
    Mlb,
    Nfl,
}
impl fmt::Display for League {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
pub struct YahooFantasyFactory {
    pub yahoo_client: YahooConnection,
    league: League,
}

impl YahooFantasyFactory {
    pub fn new_factory(league: League) -> Self {
        YahooFantasyFactory {
            yahoo_client: YahooConnection::new(),
            league,
        }
    }

    pub async fn get_league_resource(&self) -> Result<(), Error> {
        let url =
            self.yahoo_client.fantasy_sports_url.clone() + &self.league.to_string().to_lowercase();
        let response = reqwest::get(url.to_string()).await?;
        println!("{:?}", response.status());
        println!("{:?}", response.headers());

        let body = response.text().await?;
        println!("{:?}", body);

        Ok(())
    }

    pub async fn get_my_roster() -> Roster {
        let mut my_roster = Roster::new();

        let positions: Vec<Position> = vec![];
        let elias = Player::new("Elias".to_string(), "Pettersson".to_string(), positions.clone(), false, NhlFranchise::VancouverCanucks, Team::new("VAN".to_string(), 21));
        // "VAN", msf_id: 21

        let zib = Player::new("Mika".to_string(), "Zibanejad".to_string(), positions.clone(), false, NhlFranchise::NewYorkRangers, Team::new("NYR".to_string(), 9));
        // "NYR", msf_id: 9

        let nico = Player::new("Nico".to_string(), "Hischier".to_string(), positions.clone(), false, NhlFranchise::NewJerseyDevils, Team::new("NJD".to_string(), 7));
        // "NJD", msf_id: 7

        my_roster.add_player(elias);
        my_roster.add_player(zib);
        my_roster.add_player(nico);

        my_roster


    }
}
