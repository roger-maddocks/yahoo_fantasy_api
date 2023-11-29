use crate::yahoo_auth_profile::YahooConnection;
use std::fmt;
use std::fmt::Formatter;
use reqwest::Error;

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

        let url = self.yahoo_client.fantasy_sports_url.clone() + &self.league.to_string().to_lowercase();
        let response = reqwest::get(url.to_string())
            .await?;
        println!("{:?}", response.status());
        println!("{:?}", response.headers());

        let body = response.text().await?;
        println!("{:?}", body);

        Ok(())
    }
}
