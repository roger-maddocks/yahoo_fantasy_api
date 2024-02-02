use crate::models::yahoo_player::{YahooPlayer, YahooPlayers};
use crate::yahoo_auth_profile::YahooAuthClient;
use reqwest::Error;
use std::fmt;
use std::fmt::{Debug, Formatter};
use roxmltree::ExpandedName;
// use serde_xml;
use serde_json::Value;
use serde_xml_rs::from_str;
use crate::builders::roster_builder::Roster;
use crate::models::player::{NhlFranchise, Player, Position};
use crate::models::player::Position::C;
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
            .await
            .unwrap();

        let doc = roxmltree::Document::parse(&response);

        for node in doc.unwrap().descendants() {
            println!("key: {:?} | val: {:?}" , node.tag_name().name(), node.text());
        }

        // println!("roster: {:?}", response.unwrap());

        Ok(())
    }

    ///
    /// Different options to be implemented
    /// //"/league/427.l.28172/players;count=5;status=A;sort=PTS";
    /// //"/league/427.l.28172/players;status=A";
    /// //"/league/427.l.28172";
    /// //"/game/nhl";
    ///
    ///
    pub async fn get_free_agents(&mut self) -> Result<(), roxmltree::Error> {//-> Result<(), Error> {
        let url = env!["YAHOO_V2_URL"]
            .to_string() + "/league/427.l.28172/players;count=1;status=A;sort=PTS";
        let client = reqwest::Client::new();

        self.yahoo_client.generate_get_request_headers().await;

        let response = client//: YahooPlayers = client
            .get(url)
            .headers(self.yahoo_client.request_headers.clone())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        println!("result's: {:?}" , &response);
        println!("resulting no newlines: {:?}" , &response.replace("\n",""));

        let free_agents: Vec<YahooPlayer> = from_str(&response).unwrap();
        println!("faaaaa's: {:?}" , free_agents);


        // let doc = roxmltree::Document::parse(&response);

        // for node in doc.unwrap().descendants() {
        //     if node.tag_name() != ExpandedName::from("") {
        //         match node.tag_name().name() {
        //             x if Some(x) == Option::from("full") =>  println!("> {:?} ", node.text()),
        //             _ => ()
        //         };
        //     }
        // }

        // println!(" tag: {:?} | key: {:?} | val: {:?}" , node.tag_name(), node.tag_name().name(), node.text());
        // for node in doc.unwrap().descendants() {
        //     println!(" tag: {:?} | key: {:?} | val: {:?}" , node.tag_name(), node.tag_name().name(), node.text());
        // }
        //
        // for node in doc.unwrap().descendants() {
        //     if node.tag_name() != ExpandedName::from("") {
        //         match node.text() {
        //             x if x == Option::from("\n    ") =>  println!("> {:?} Parent: ", node.tag_name().name()),
        //             x if x == Option::from("\n     ") =>  println!(">> {:?} Object: ", node.tag_name().name()),
        //             x if x == Option::from("") => (),
        //             _ => println!("key: {:?} | val: {:?}" , node.tag_name().name(), node.text()),
        //         };
        //     }
        // }

        // for node in doc.unwrap().descendants() {
        //     // node.fmt(&mut YahooPlayer);
        //     println!("{:?}", node);
        //     println!("key: {:?} | val: {:?}" , node.tag_name().name(), node.text());
        //
        // }
        // println!(r#"test: {:#?}"#, doc.unwrap().root().descendants());
        // for token in xmlparser::Tokenizer::from(&*something) {
        //     println!("--{:#?}", token);
        // }
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

        let position = vec![C];
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
