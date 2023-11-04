use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, TokenUrl,
};
use std::env;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use url::Url;

use reqwest::Error;
use crate::fantasy_week::FantasyWeek;
use crate::team::Team;
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

#[tokio::main]
async fn main () -> Result<(), Error> {
    for i in 4 ..= 4  {
        let this_week = FantasyWeek::new(i, i);
        weekly_data_factory::get_loaded_schedule_report(i, &this_week).await;
    }

    Ok(())
}
// Define your Yahoo Fantasy Sports API credentials
const CLIENT_ID: &str = env!["YAHOO_CLIENT_ID"];
const CLIENT_SECRET: &str = env!["YAHOO_CLIENT_SECRET"];
const REDIRECT_URI: &str = "https://www.google.com"; // Your redirect URI
// const REDIRECT_URI: &str = "http://localhost:8080/callback"; // Your redirect URI

// Initialize an OAuth2 client with your credentials
pub fn create_yahoo_oauth_client() -> BasicClient {
    let client_id = ClientId::new(CLIENT_ID.to_string());
    let client_secret = ClientSecret::new(CLIENT_SECRET.to_string());

    BasicClient::new(
        client_id,
        Some(client_secret),
        AuthUrl::new(Url::parse("https://api.login.yahoo.com/oauth2/request_auth")?),
        Some(TokenUrl::new(Url::parse("https://api.login.yahoo.com/oauth2/get_token")?)),
    )
        .set_redirect_uri(RedirectUrl::new(Url::parse(REDIRECT_URI)?))
}

// Start the OAuth2 authorization flow
pub fn initiate_oauth_flow() -> String {
    let (auth_url, csrf_state) = create_yahoo_oauth_client().authorize_url(CsrfToken::new_random);

    auth_url.to_string()
}

// Handle the OAuth2 callback, exchange the authorization code for a token
pub fn handle_callback(code: &str) -> Result<oauth2::Token, String> {
    let token = create_yahoo_oauth_client()
        .exchange_code(AuthorizationCode::new(code.to_string()))
        .request(http_client)
        .map_err(|e| e.to_string())?;

    Ok(token)
}