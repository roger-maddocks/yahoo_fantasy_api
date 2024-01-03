use crate::models::yahoo_player::YahooPlayers;
use crate::yahoo_auth_profile::YahooAuthClient;
use reqwest::Error;
use std::fmt;
use std::fmt::Formatter;
// use serde_xml;
use serde_json::Value;
use crate::builders::roster_builder::Roster;
use crate::models::player::{NhlFranchise, Player, Position};
use crate::models::player::Position::Center;
use crate::models::team::Team;
use crate::builders::yahoo_auth_client_builder::YahooAuthClientBuilder;

#[derive(Debug, Copy, Clone)]
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
#[derive(Clone)]
pub struct YahooFantasyFactory {
    pub yahoo_client: YahooAuthClient,
    league: League,
}

impl YahooFantasyFactory {
    pub async fn new_factory(league: League) -> YahooFantasyFactory {
        YahooFantasyFactory {
            yahoo_client: YahooAuthClientBuilder::new().build().await,
            league,
        }
    }

    // pub async fn get_top_ten_free_agents(&self) -> Result<(), Error> {
    //    //get free agents, then take each player and get last 5 game scores
    //    //  self.get_free_agents()
    // }

    pub async fn get_my_roster(&self) -> Result<(), Error> {

        let url = env!["YAHOO_V2_URL"].to_string() + "/team/427.l.28172.t.7/roster;";
        let client = reqwest::Client::new();

        let response = client
            .get(url)
            .headers(self.yahoo_client.request_headers.clone())
            .send()
            .await
            .unwrap()
            .text()
            .await;

        println!("roster: {:?}", response.unwrap());

        Ok(())
    }

    pub async fn get_free_agents(&mut self) -> Result<(), serde_json::Error> {//-> Result<(), Error> {
        let url = env!["YAHOO_V2_URL"].to_string() + "/league/427.l.28172/players;status=A";//"/league/427.l.28172"; //
        let client = reqwest::Client::new();
        self.yahoo_client.generate_get_request_headers().await;

        // println!("Debug headers: {:?}", self.yahoo_client.request_headers.clone());
        let response = client//: YahooPlayers = client
            .get(url)
            .headers(self.yahoo_client.request_headers.clone())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let mut something = "".to_string();
        for line in response.lines() {
            // println!("{:?}", &line);
           something += line.trim();
        }
        // println!("{:?}", &response);
        println!(r#"{:?}"#, &something);

        // let doc: YahooPlayers = serde_xml::from_str(&response.to_owned()).unwrap();

        // let doc: YahooPlayers = serde_xml::from_str(&something).unwrap();
        // let doc = quick_xml::Reader::from_str(&response);
        // let doc = roxmltree::Document::parse(&something);
        for token in xmlparser::Tokenizer::from(&*something) {
            // println!("--{:#?}", token.unwrap());
        }
        // response.lines().for_each(move |x| -> let a = xmlparser::Tokenizer::from(x));
        // println!("response.xml(): {:?}", doc);

        // let parsed: Value = serde_json::from_str(&response.to_string().clone())?;

        // let response = client
        //     .get(url)
        //     .headers(self.yahoo_client.request_headers.clone())
        //     .send()
        //     .await
        //     .unwrap()
        //     .json()
        //     .await
        //     .unwrap();

        // println!("roster: {:#?}", parsed);

        // let body = response.text().await?;
        // println!("{:?}", body);

        Ok(())
    }
    pub async fn get_league_resource(&self) -> Result<String, Error> {
        let url = env!["YAHOO_V2_URL"].to_string() + "/users;use_login=1/games;game_keys=nhl/teams";
        let client = reqwest::Client::new();

        let response = client
            .get(url)
            .headers(self.yahoo_client.request_headers.clone())
            .send()
            .await
            .unwrap()
            .text()
            .await;

        response
    }

    pub async fn get_test_roster() -> Roster {
        let mut my_roster = Roster::new();

        let position = vec![Center];
        let elias = Player::new("Elias".to_string(), "Pettersson".to_string(), position.clone(), false, NhlFranchise::VancouverCanucks, Team::new("VAN".to_string(), 21), "".to_string());
        // "VAN", msf_id: 21

        let zib = Player::new("Mika".to_string(), "Zibanejad".to_string(), position.clone(), false, NhlFranchise::NewYorkRangers, Team::new("NYR".to_string(), 9), "".to_string());
        // "NYR", msf_id: 9

        let nico = Player::new("Nico".to_string(), "Hischier".to_string(), position.clone(), false, NhlFranchise::NewJerseyDevils, Team::new("NJD".to_string(), 7), "".to_string());
        // "NJD", msf_id: 7

        my_roster.add_player(elias);
        my_roster.add_player(zib);
        my_roster.add_player(nico);

        my_roster


    }
}
