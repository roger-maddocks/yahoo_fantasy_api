use crate::yahoo_auth_profile::YahooConnection;
use std::fmt;
use std::fmt::Formatter;
use oauth2::ErrorResponse;
use reqwest::Error;
use serde::{Deserialize, Serialize};

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
#[derive(Default, Serialize, Deserialize)]
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

    pub async fn get_league_resource(&self) -> Result<YahooFantasyFactory, dyn ErrorResponse> {
        let url = self.yahoo_client.fantasy_sports_url.clone() + &self.league.to_string().to_lowercase();

        println!("get_league_resource url: {:?}", url);

        let client = reqwest::Client::new();
        let response = client
            .post(&self.yahoo_client.fantasy_sports_url)
            .form(&self.yahoo_client.refresh_token_params)
            .headers(self.yahoo_client.headers.clone())
            .send()
            .await
            .expect("Get token error!")
            .json()
            .await
            .unwrap();

        // Some(response)
        response

        // let body = response.text().await?;
        // println!("{:?}", body);
        //
        // Ok(())
    }
}
