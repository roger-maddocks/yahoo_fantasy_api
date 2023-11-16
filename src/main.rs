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
mod yahoo_fantasy_factory;

#[tokio::main]
async fn main () -> Result<(), Box<dyn Error>> {

    yahoo_auth_profile::YahooConnection::get_redirect_url_for_auth_code();
    let auth = yahoo_auth_profile::YahooConnection::new();

    print!("url: {:?}", auth.token_url);
    println!("{:?}", auth.token_params);

    let client = reqwest::Client::new();
    let response = client
        .post(auth.token_url)
        .form(&auth.refresh_token_params)
        .headers(auth.headers)
        .send()
        .await?
        .text()
        .await
        .unwrap();

    println!("{:#?}", response);

    // for i in 5 ..= 5  {
    //     let this_week = FantasyWeek::new(i, i);
    //     weekly_data_factory::get_loaded_schedule_report(i, &this_week).await;
    // }

    Ok(())
}

