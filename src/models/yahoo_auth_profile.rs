use anyhow;
use anyhow::Error;
use oauth2;
use oauth2::http::{HeaderMap, HeaderValue};
use oauth2::url;
use oauth2::{ClientId, ClientSecret, RevocableToken};
use std::str::FromStr;
use serde::Serialize;
use thiserror::Error;
use url::{form_urlencoded, Url};
use crate::models::scheduled_games::Game;

pub struct YahooConnection {
    pub(crate) token_params: YahooTokenRequest,
    pub(crate) auth_params: YahooAuthRequest,
    pub(crate) refresh_token_params: YahooRefreshTokenRequest,
    pub(crate) token_url: String,
    pub(crate) auth_url: String,
    pub(crate) fantasy_sports_url: String,
    pub(crate) access_token: String,
    pub(crate) auth_headers: HeaderMap,
    pub(crate) get_headers: HeaderMap,
}

impl YahooConnection {
    pub fn new() -> Self {
        YahooConnection {
            token_params: YahooTokenRequest::new(),
            auth_params: YahooAuthRequest::new(),
            refresh_token_params: YahooRefreshTokenRequest::new(),
            token_url: env!("YAHOO_TOKEN_URL").to_string(),
            auth_url: env!("YAHOO_AUTH_ENDPOINT").to_string() + "?",
            fantasy_sports_url: env!("YAHOO_V2_URL").to_string() + "/",
            access_token: "".to_string(),
            auth_headers: generate_refresh_token_headers(
                ClientId::new(env!("YAHOO_CLIENT_ID").to_string()),
                Some(ClientSecret::new(env!("YAHOO_CLIENT_SECRET").to_string())).unwrap(),
                Default::default(),
            ),
            get_headers: generate_get_request_headers(Default::default())
        }
    }

    pub fn get_redirect_url_for_auth_code() {
        let auth = YahooConnection::new();
        let encoded_string =
            serde_urlencoded::to_string(&auth.auth_params)
                .expect("serializing issue!");
        let url = auth.auth_url.to_string() + &encoded_string;
        println!("{:#?}", url);
    }
    pub async fn get_access_token(&self) -> Option<String> {
        let client = reqwest::Client::new();
        let response= client
            .post(&self.token_url)
            .form(&self.refresh_token_params)
            .headers(self.auth_headers.clone())
            .send()
            .await
            .expect("Get token error!")
            .text()
            .await
            .unwrap();

        Some(response)
    }
    pub async fn get_access_token_json(&self) -> YahooRefreshTokenResponse {
        let client = reqwest::Client::new();
        let response  = client
            .post(&self.token_url)
            .form(&self.refresh_token_params)
            .headers(self.auth_headers.clone())
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        response
    }

    // pub async fn get_access_token(&self) -> Option<YahooRefreshTokenResponse> {
    //     let client = reqwest::Client::new();
    //     let response: YahooRefreshTokenResponse = client
    //         .post(&self.token_url)
    //         .form(&self.refresh_token_params)
    //         .headers(self.headers.clone())
    //         .send()
    //         .await
    //         .expect("Get token error!")
    //         .json()
    //         .await
    //         .unwrap();
    //
    //     Some(response)
    // }
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
#[derive(Default, serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct YahooRefreshTokenResponse {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
    pub token_type: Option<String>,
}

impl YahooRefreshTokenResponse {
    pub fn new() -> Self {
        YahooRefreshTokenResponse {
            access_token: None,
            refresh_token: None,
            expires_in: None,
            token_type: None,
        }
    }
}


///Yahoo required fields for Get request
#[derive(Default, serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct YahooGetRequest {
    pub client_id: String,
    pub client_secret: String,
    pub authorization: String,
    pub refresh_token: String,
}
impl YahooGetRequest {
    pub fn new() -> Self {
        YahooGetRequest {
            client_id: env!("YAHOO_CLIENT_ID").to_string(),
            client_secret: env!("YAHOO_CLIENT_SECRET").to_string(),
            authorization: "".to_string(),
            refresh_token: "".to_string(),
        }
    }
}

#[derive(Debug, Serialize, serde::Deserialize, Clone )]
#[serde(rename_all = "camelCase")]
pub struct YahooResponses {
    pub tokens: Vec<YahooRefreshTokenResponse>
}

fn generate_refresh_token_headers(
    client_id: ClientId,
    secret: ClientSecret,
    mut headers: HeaderMap,
) -> HeaderMap {
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

fn generate_get_request_headers(
    mut headers: HeaderMap, ) -> HeaderMap {

    headers.append(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", "SPQzatSYuwuXs_HWy5qhB0tKh9vj5Qyp1n9tDhc7WIwTXojO_cw77YJAX0mvDxlaghHxRoNmOzrm_Hdf9ktTiabtnaBV5bThCLi3f0LjlbJuj4sN2iiEpUdVrrf3cQE1L0TuOljGTJ7ldhnW.UoIl.APSV2YCXEaJ7caZaCtooqPplBLs6oy9fU_q.GnYayFTv_BxdL_W9EjrIBBrHGX.z.YUViWKtXp1U0ZloaDXkZIw0h07yPWcV1BgkwtwfWT74UjrwITTLx0MbOpqPU3nkNsNCHaGznGvT785mwR7I3zxmPCS_6x9O4pDSs.z76sJtLJD_4N7_oa8DXNs9K7V_BycqBuTtCyWaOSdzeRT5tncG9z0TG3iEVTp_SUdCznHGKvzNxauDOv3GiiRxMzsPM.K0Mj1n.qCE_fbM.x46yyVb5J4Xc5Z3lDVtvMOI7KJdMdevqQVM2AfzGUK2shwOyIRJPkWX1S1kwhkMutsa.I9ROgNAuqT9QEwhBtYHWwtN_UflW_Ll3y.HtTrc_r4NkV5oau.7yaY.eC8TRGclmSXh5A_DotSeOoSGtJC_hd5UBL4Qj2YMoMdD6_vQ1P5pSIvVBwtQeCNks3VQQ3gqxImRDaF.DfLefcCZi7I6d1k0pOgrUfXSOjbH13b6gGYeqwF9yixnPUIeS5eXCG498PoJ6cWICE0SimL1l.GR7Nh0q07iifsliAuQtjGH5l7mtw5PaizXzcBp_wHbg.m31Se0wc_FH2x.aBXWiwlp41pZ7ykG77AYMnNkrQivRM7ZBm8SiAUceV6ZQq1T_oNuX2pR.dL9SIH288FYSbFW_kJmImNPLCAiFMFKXrebp09cefdxMYsuhqQ.mecg1Au9pAo47mB.DD65oCgrhvwUYxF2Q1zGPfWrbVHepJ8uSkyqy0kNZj2_9OH.EkJgRj9welNJm_wPYRDff7wCJt.AuVVi.AhY8xKpe8kfEL3j0E4C_IqmhLCf__y6cC")).unwrap(),
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
