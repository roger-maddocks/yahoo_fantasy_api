use std::marker::PhantomData;
use std::str::FromStr;
use oauth2::{AuthorizationCode, AuthType, AuthUrl,  ClientId, ClientSecret, HttpRequest, RedirectUrl, Scope, TokenResponse, TokenUrl, ErrorResponse, TokenType, TokenIntrospectionResponse, RevocableToken, DeviceAuthorizationUrl, IntrospectionUrl, RevocationUrl};
// use reqwest::{Client, ClientBuilder, Method};
use thiserror::Error;
use oauth2;
use anyhow;
use AuthType::BasicAuth;
use oauth2::basic::BasicClient;
use oauth2::http::{HeaderMap, HeaderValue};
use oauth2::reqwest::async_http_client;
use oauth2::url;
use url::{form_urlencoded, Url};

#[derive(Error, Debug)]
pub enum YahooError {
    #[error("fetching the data from yahoo! finance failed")]
    FetchFailed(String),
    #[error("deserializing response from yahoo! finance failed")]
    DeserializeFailed(#[from] serde_json::Error),
    #[error("connection to yahoo! finance server failed")]
    ConnectionFailed(#[from] reqwest::Error),
    #[error("yahoo! finance return invalid JSON format")]
    InvalidJson,
    #[error("yahoo! finance returned an empty data set")]
    EmptyDataSet,
    #[error("yahoo! finance returned inconsistent data")]
    DataInconsistency,
    #[error("construcing yahoo! finance client failed")]
    BuilderFailed,
}


#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct YahooParams {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    code: String,
    grant_type: String,
}
#[derive(Default)]
pub struct YahooEncode {
    pub(crate) params: YahooParams,
    pub(crate) headers: HeaderMap,
}
impl YahooParams {
    pub fn new () -> Self  {
        YahooParams {
            client_id: env!("YAHOO_CLIENT_ID").to_string(),
            client_secret: env!("YAHOO_CLIENT_SECRET").to_string(),
            grant_type: "authorization_code".to_string(),
            redirect_uri: "oob".to_string(),//"https://www.google.com".to_string(),
            code: "2y88gm6".to_string(),
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

    headers.append( "Authorization-Code", HeaderValue::from_str(&format!("Basic {}", &b64_credential)).unwrap(), );
    headers.append("Content-Type", "application/x-www-form-urlencoded".to_string().parse().unwrap());

    headers
}

//"{\"access_token\":\"VucdW_mYuwvxJJW4gFVXzVpc9p0QU6cRozkx1xS.69Wc3dR42GJnMZpN6WvXFRwsUv7bQcPjlxgOYQAyqutICMYLSD3pW6pERzfNCkHtgoItrmuyOZHOjnMCw0jFZxxOZe54Ij6JK4rTAc8Q.WKmwQhhydl5HzEOoBPaQCgZiYnc4PeOgh40CcIG.a6pkLkVXAV2xlR_UqeutUOI81sW_cr9eGbomkJt7eALH1YQZasMmbFUN_QdOAJyTREWzDeSSOjDi921ZV7kRzS.vqePoa1AELksHtY2.qgFT8kaBKJIM9YLfOgMdUoHdR98jM8hW.86uwotvLLIfDs2W_sHDi9Nt9eihYOAgHU946sHENsdHxhz8gFLDWCKwjddh1C9YxrJys1184JTJ1_wQ_.V66J7x2FoRfM_TZW3C8qR55VurrE60RjQVAtFGurJoxZ2U2kakEsVlvDacXWsz5kmiEwy7_lJxhK8DGyhg62riyUMObfpnsjJ8IAcGD8rVq9XPcd5vKl86TYULE1BoYaqjojf6PHRxpkVORBwQIH.71swbMErF5PNiF0jZ08.EI3GLl_xNh7.op6PhLXkqVLEGif.gIH_39l9IDRfYzgO3LgmMGk5tm6_LindeVKTmx0_aNsm_IMNN6GA7rV4FSJCBgy6ChdiliJFk9oSqaQIhwm3.xD_PlDhHkW1cQhZ.ANbKxDt67aSQThNU_ZmlmkXebcmFRnafVqCj44bOWWs_nfrwwcQn9Sy5Zj0UwchovznB8sSaBbrdZZjBrcKt.u5NCrj7vQL9XSp00TUwAr7aGGhRq3W7.Puq5iLkfq41xPN7HfYVbYKbWZC1O82_KnUtarujfWXT4wHW7HVjalwcw73lGhTA8YN01HiFt04XGI3_TEL3dYHi3ilSu6viBZFZC9R6PSaoM3lnAoU1WiB8d8zyH9h.b23ubX5XRBmxdfPOHJhVoGw8PgqWDCf0en1kPGZOQERevhuUilM\",\"refresh_token\":\"ABozVWW1mLQ5sPJ_JjhZkKMu1pHU~000~yCpm41AyJxoTlNjhjmJCmCvA1Fmj9LjgQr.E\",\"expires_in\":3600,\"token_type\":\"bearer\"}"
