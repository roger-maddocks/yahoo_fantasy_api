use crate::yahoo_auth_profile::YahooConnection;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
enum League {
    NHL,
    NBA,
    MLB,
    NFL,
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
    pub fn new_nhl_factory() -> Self {
        YahooFantasyFactory {
            yahoo_client: YahooConnection::new(),
            league: League::NHL,
        }
    }

    pub fn get_league_resource() {}
}
