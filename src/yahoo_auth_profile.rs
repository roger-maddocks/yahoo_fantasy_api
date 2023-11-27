use anyhow;
use anyhow::Error;
use oauth2;
use oauth2::http::{HeaderMap, HeaderValue};
use oauth2::url;
use oauth2::{ClientId, ClientSecret, RevocableToken};
use std::str::FromStr;
use thiserror::Error;
use url::{form_urlencoded, Url};

pub struct YahooConnection {
    pub(crate) auth_params: YahooAuthRequest,
    pub(crate) token_params: YahooTokenRequest,
    pub(crate) refresh_token_params: YahooRefreshTokenRequest,
    pub(crate) bearer_credentials: YahooRefreshTokenResponse,
    pub(crate) headers: HeaderMap,
    pub(crate) token_url: String,
    pub(crate) auth_url: String,
    pub(crate) fantasy_sports_url: String,
    // pub(crate) access_token: String,
}

impl YahooConnection {
    pub fn new() -> Self {
        YahooConnection {
            token_params: YahooTokenRequest::new(),
            auth_params: YahooAuthRequest::new(),
            refresh_token_params: YahooRefreshTokenRequest::new(),
            bearer_credentials: Default::default(),
            headers: my_encode(
                ClientId::new(env!("YAHOO_CLIENT_ID").to_string()),
                Some(ClientSecret::new(env!("YAHOO_CLIENT_SECRET").to_string())).unwrap(),
                Default::default(),
            ),
            token_url: env!("YAHOO_TOKEN_URL").to_string(),
            auth_url: env!("YAHOO_AUTH_ENDPOINT").to_string(),
            fantasy_sports_url: env!("YAHOO_V2_URL").to_string(),
            // refresh_token: env!("YAHOO_FANTASY_REFRESH_TOKEN").to_string(),
            // access_token: "".to_string(),
        }
    }

    pub fn get_redirect_url_for_auth_code() {
        let auth = YahooConnection::new();
        let encoded_string =
            serde_urlencoded::to_string(&auth.auth_params).expect("serializing issue!");
        let url = auth.auth_url.to_string() + &encoded_string;
        println!("{:#?}", url);
    }

    pub async fn get_access_token(&self) {
        let client = reqwest::Client::new();

        let response:YahooRefreshTokenResponse = client
            .post(&self.fantasy_sports_url)
            .form(&self.bearer_credentials)
            .headers(self.headers.clone())
            .send()
            .await
            .expect("Get token error!")
            .json()
            .await
            .unwrap();

    }
}

///Yahoo required fields for initiating auth request
#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct YahooAuthRequest {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    response_type: String,
}
impl YahooAuthRequest {
    pub fn new() -> Self {
        YahooAuthRequest {
            client_id: env!("YAHOO_CLIENT_ID").to_string(),
            client_secret: env!("YAHOO_CLIENT_SECRET").to_string(),
            redirect_uri: "oob".to_string(),
            response_type: "code".to_string(),
        }
    }
}

///Yahoo required fields for acquiring access token
#[derive(Default, serde::Serialize, serde::Deserialize, Debug)]
pub struct YahooTokenRequest {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    code: String,
    grant_type: String,
}
impl YahooTokenRequest {
    pub fn new() -> Self {
        YahooTokenRequest {
            client_id: env!("YAHOO_CLIENT_ID").to_string(),
            client_secret: env!("YAHOO_CLIENT_SECRET").to_string(),
            grant_type: "authorization_code".to_string(),
            redirect_uri: "oob".to_string(),
            code: env!("YAHOO_AUTH_CODE").to_string(),
        }
    }
}

///Yahoo required fields for refreshing access token
#[derive(Default, serde::Serialize, serde::Deserialize, Debug)]
pub struct YahooRefreshTokenRequest {
    grant_type: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    refresh_token: String,
}
impl YahooRefreshTokenRequest {
    pub fn new() -> Self {
        YahooRefreshTokenRequest {
            grant_type: "refresh_token".to_string(),
            client_id: env!("YAHOO_CLIENT_ID").to_string(),
            client_secret: env!("YAHOO_CLIENT_SECRET").to_string(),
            redirect_uri: "oob".to_string(), //"https://www.google.com".to_string(),
            refresh_token: env!("YAHOO_REFRESH_TOKEN").to_string(), //env!("YAHOO_REFRESH_TOKEN").to_string()//"ABozVWW1mLQ5sPJ_JjhZkKMu1pHU~000~yCpm41AyJxoTlNjhjmJCmCvA1Fmj9LjgQr.E".to_string()//
        }
    }
}

/// Yahoo's response values when requesting token refresh
#[derive(Default, serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct YahooRefreshTokenResponse {
    access_token: String,
    refresh_token: String,
    expires_in: u64,
    token_type: String,
}

fn my_encode(client_id: ClientId, secret: ClientSecret, mut headers: HeaderMap) -> HeaderMap {
    let urlencoded_id: String = form_urlencoded::byte_serialize(&client_id.as_bytes()).collect();
    let urlencoded_secret: String =
        form_urlencoded::byte_serialize(secret.secret().as_bytes()).collect();
    let b64_credential = base64::encode(&format!("{}:{}", &urlencoded_id, urlencoded_secret));

    headers.append(
        "Authorization-Code",
        HeaderValue::from_str(&format!("Basic {}", &b64_credential)).unwrap(),
    );
    headers.append(
        "Content-Type",
        "application/x-www-form-urlencoded"
            .to_string()
            .parse()
            .unwrap(),
    );

    headers
}

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

// APFQVmXz1I1aQopVfPek1xpc_6V4~000~yCpm41AyJxoTlNjhjmJCmCvA1Fmj9LjgQr.E
