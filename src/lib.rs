use anyhow;
use oauth2::{AuthorizationCode, AuthUrl, Client, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, StandardRevocableToken, TokenResponse, TokenUrl};
use oauth2::basic::{BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse, BasicTokenType};
use oauth2::reqwest::{async_http_client, http_client};
use anyhow::Error;
use url::Url;


// #[tokio::main]
// pub async fn yahoo_connector() -> Result<(), Error> {
//
//     let client =
//         BasicClient::new(
//             ClientId::new(env!["YAHOO_CLIENT_ID"].to_string()),
//             Some(ClientSecret::new(env!["YAHOO_CLIENT_SECRET"].to_string())),
//             AuthUrl::new(env!["YAHOO_AUTH_ENDPOINT"].to_string())?,
//             Some(TokenUrl::new(env!["YAHOO_TOKEN_URL"].to_string())?))
//         ;
//     let token_result = client
//         .exchange_client_credentials()
//         .add_scope(Scope::new("read".to_string()))
//         .request(http_client)?;
//     println!("token result: {:?}", token_result);
//
//
//     Ok(())
// }
// ;//.set_redirect_uri(RedirectUrl::new("https://www.google.com".to_string())?);

// let (_pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();

// let (auth_url, csrf_token) = client
//     .authorize_url(CsrfToken::new_random)
//     .add_scope(Scope::new("read".to_string()))
//     .add_scope(Scope::new("write".to_string()))
//     .set_pkce_challenge(_pkce_challenge)
//     .url();

