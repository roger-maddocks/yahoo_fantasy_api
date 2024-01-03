use oauth2::{ClientId, ClientSecret};
use oauth2::http::HeaderMap;
use crate::models::yahoo_auth_profile::{YahooAuthClient, YahooAuthRequest, YahooRefreshTokenRequest, YahooRefreshTokenResponse, YahooTokenRequest};

#[derive(Default)]
pub struct YahooAuthClientBuilder {
    pub token_params: YahooTokenRequest,
    pub auth_params: YahooAuthRequest,
    pub refresh_token_params: YahooRefreshTokenRequest,
    pub token_url: String,
    pub auth_url: String,
    pub fantasy_sports_url: String,
    pub request_headers: HeaderMap,
    pub access_token: String,
    pub auth_headers: HeaderMap,

}
impl YahooAuthClientBuilder {
    pub fn new() -> YahooAuthClientBuilder {
        YahooAuthClientBuilder {
            token_params: YahooTokenRequest::new(),
            auth_params: YahooAuthRequest::new(),
            refresh_token_params: YahooRefreshTokenRequest::new(),
            token_url: env!("YAHOO_TOKEN_URL").to_string(),
            auth_url: env!("YAHOO_AUTH_ENDPOINT").to_string() + "?",
            fantasy_sports_url: env!("YAHOO_V2_URL").to_string() + "/",
            request_headers: Default::default(),
            access_token: "".to_string(),
            auth_headers: crate::models::yahoo_auth_profile::generate_refresh_token_headers(
                ClientId::new(env!("YAHOO_CLIENT_ID")
                    .to_string()),
                Some(ClientSecret::new(env!("YAHOO_CLIENT_SECRET")
                    .to_string()))
                    .unwrap()
            ),
        }
    }
    pub async fn build(mut self) -> YahooAuthClient {
        self.refresh_access_token().await;
        YahooAuthClient {
            token_params: self.token_params,
            auth_params: self.auth_params,
            refresh_token_params: self.refresh_token_params,
            token_url: self.token_url,
            auth_url: self.auth_url,
            fantasy_sports_url: self.fantasy_sports_url,
            request_headers: self.request_headers,
            auth_headers: self.auth_headers,
            access_token:self.access_token
        }
    }

    pub async fn refresh_access_token(&mut self) {
        let client = reqwest::Client::new();
        let response: YahooRefreshTokenResponse  = client
            .post(&self.token_url)
            .form(&self.refresh_token_params)
            .headers(self.auth_headers.clone())
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        self.access_token = response.access_token.unwrap();
        println!("{:?}", self.access_token);

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
