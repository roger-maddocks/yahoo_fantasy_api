use std::time::Duration;
use crate::fantasy_week::FantasyWeek;
use serde_urlencoded;
use crate::team::Team;
use oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, AuthType, AuthUrl, ClientId, ClientSecret, HttpRequest, RedirectUrl, Scope, TokenResponse, TokenUrl, ErrorResponse, TokenType, TokenIntrospectionResponse, RevocableToken, DeviceAuthorizationUrl, IntrospectionUrl, RevocationUrl, Client};
use thiserror::Error;
use AuthType::BasicAuth;
use reqwest;
use oauth2;
use anyhow;
// use anyhow::Error;
use oauth2::basic::BasicClient;
use oauth2::DeviceCodeErrorResponseType::Basic;
use oauth2::reqwest::async_http_client;
use oauth2::url;
use url::{form_urlencoded, Url};
use std::error::Error;
use std::ops::Add;
use base64::encode;
use oauth2::http::header::AUTHORIZATION;
use oauth2::http::{HeaderMap, HeaderValue, Response};
use oauth2::RequestTokenError::Request;
use serde::Serialize;

pub mod roster_builder;
mod my_sports_feed_profile;
mod scheduled_games;
mod regular_season;
mod team;
mod player;
mod weekly_data_factory;
mod report;
mod fantasy_week;
mod collision_report;
mod yahoo_auth_profile;

#[tokio::main]
async fn main () -> Result<(), Box<dyn Error>> {

    let auth = YahooEncode::new();
    let auth2 = YahooParams::new();

    let encoded_string = serde_urlencoded::to_string(&auth.params).expect("serializing issue!");

    println!("{:#?}", encoded_string);

    let client = reqwest::Client::new();
    let response = client
        .post(env!("YAHOO_TOKEN_URL"))
        .form(&auth.params)
        .headers(auth.headers)
        .send()
        .await?
        .text()
        .await
        .unwrap();

    println!("{:#?}", response);
    // let mut yahoo_request = Request::new(reqwest::Method::POST, env!("YAHOO_TOKEN_URL"));

    //right after first await for testing
    //     ;
    //
    // println!("{:#?}", response);
    //
    // println!("{:#?}", response

    // for i in 5 ..= 5  {
    //     let this_week = FantasyWeek::new(i, i);
    //     weekly_data_factory::get_loaded_schedule_report(i, &this_week).await;
    // }

    Ok(())
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
struct YahooParams {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    code: String,
    grant_type: String,
}
#[derive(Default)]
struct YahooEncode {
    params: YahooParams,
    headers: HeaderMap,
}
impl YahooParams {
    pub fn new () -> Self  {
        YahooParams {
            client_id: env!("YAHOO_CLIENT_ID").to_string(),
            client_secret: env!("YAHOO_CLIENT_SECRET").to_string(),
            grant_type: "authorization_code".to_string(),
            redirect_uri: "oob".to_string(),//"https://www.google.com".to_string(),
            code: env!("YAHOO_AUTH_CODE").to_string(),
        }
    }
}

impl YahooEncode {
    pub fn new() -> Self {
        YahooEncode {
            params: YahooParams::new(),
            headers: my_encode(ClientId::new(env!("YAHOO_CLIENT_ID").to_string()),
                               Some(ClientSecret::new( env!("YAHOO_CLIENT_SECRET").to_string())).unwrap(),
                               Default::default()),
        }
    }
}

fn my_encode (client_id: ClientId, secret: ClientSecret, mut headers: HeaderMap) -> HeaderMap {
    let urlencoded_id: String =
        form_urlencoded::byte_serialize(&client_id.as_bytes()).collect();
    let urlencoded_secret: String =
        form_urlencoded::byte_serialize(secret.secret().as_bytes()).collect();
    let b64_credential =
        base64::encode(&format!("{}:{}", &urlencoded_id, urlencoded_secret));

    headers.append( "Authorization_Code", HeaderValue::from_str(&format!("Basic {}", &b64_credential)).unwrap(), );
    headers.append("Content-Type", "application/x-www-form-urlencoded".to_string().parse().unwrap());

    headers
}



// let mut yahoo_headers =
// my_encode(ClientId::new(env!("YAHOO_CLIENT_ID").to_string()),
// Some(ClientSecret::new( env!("YAHOO_CLIENT_SECRET").to_string())).unwrap(),
// Default::default());
// yahoo_headers.append("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());
//
// let mut data = env!["YAHOO_TOKEN_URL"].to_owned() + &"?".to_string();
// println!("{:#?}", yahoo_headers);
//
//
// let client =  BasicClient::new(
// ClientId::new(env!("YAHOO_CLIENT_ID").to_string().to_string()),
// Some(ClientSecret::new( env!("YAHOO_CLIENT_SECRET").to_string())),
// AuthUrl::new(env!("YAHOO_TOKEN_URL").to_string()).unwrap(),
// Some(TokenUrl::new(env!("YAHOO_TOKEN_URL").to_string()).unwrap())
// );
//
// let token = client.exchange_client_credentials();






