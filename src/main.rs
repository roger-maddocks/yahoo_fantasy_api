use std::time::Duration;
use crate::fantasy_week::FantasyWeek;
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
use oauth2::http::header::AUTHORIZATION;
use oauth2::http::{HeaderMap, HeaderValue, Response};


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

    let mut yahoo_headers =
        encode(ClientId::new(env!("YAHOO_CLIENT_ID").to_string()),
               Some(ClientSecret::new( env!("YAHOO_CLIENT_SECRET").to_string())).unwrap(),
               Default::default());
    yahoo_headers.append("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());
    // yahoo_headers.append("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_13_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/72.0.3626.109 Safari/537.36'".to_string().parse().unwrap());

    let mut data = env!["YAHOO_TOKEN_URL"].to_owned() + &"?".to_string();
    data.push_str("client_id=");
    data.push_str(env!("YAHOO_CLIENT_ID"));
    data.push_str("&");

    data.push_str("client_secret=");
    data.push_str(env!("YAHOO_CLIENT_SECRET"));
    data.push_str("&");

    data.push_str("redirect_uri=");
    data.push_str("oob");
    data.push_str("&");

    data.push_str("code=");
    data.push_str(env!("YAHOO_AUTH_CODE"));
    data.push_str("&");

    data.push_str("grant_type=");
    data.push_str("authorization_code");

    yahoo_headers.append("Content-Length", HeaderValue::from(data.len()));

    println!("{:#?}", yahoo_headers);

    let mut yahoo_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap();

    let response = yahoo_client
        .post( &data)
        .headers(yahoo_headers)
        .send()
        .await
        .unwrap()
        ;

    println!("{:#?}", response);

    let this = response.text()
        .await
        .unwrap();
    println!("{:#?}", this);

        // .basic_auth(env!("YAHOO_CLIENT_ID").to_string(), Some(env!("YAHOO_CLIENT_SECRET").to_string()));
//
//



    // for i in 5 ..= 5  {
    //     let this_week = FantasyWeek::new(i, i);
    //     weekly_data_factory::get_loaded_schedule_report(i, &this_week).await;
    // }

    Ok(())
}

fn encode (client_id: ClientId, secret: ClientSecret, mut headers: HeaderMap) -> HeaderMap {
    let urlencoded_id: String =
        form_urlencoded::byte_serialize(&client_id.as_bytes()).collect();
    let urlencoded_secret: String =
        form_urlencoded::byte_serialize(secret.secret().as_bytes()).collect();
    let b64_credential =
        base64::encode(&format!("{}:{}", &urlencoded_id, urlencoded_secret));

    headers.append(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Basic {}", &b64_credential)).unwrap(),
    );

    headers
}



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






