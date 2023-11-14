use std::marker::PhantomData;
use std::str::FromStr;
use oauth2::{AuthorizationCode, AuthType, AuthUrl,  ClientId, ClientSecret, HttpRequest, RedirectUrl, Scope, TokenResponse, TokenUrl, ErrorResponse, TokenType, TokenIntrospectionResponse, RevocableToken, DeviceAuthorizationUrl, IntrospectionUrl, RevocationUrl};
// use reqwest::{Client, ClientBuilder, Method};
use thiserror::Error;
use oauth2;
use anyhow;
use AuthType::BasicAuth;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::url;
use url::Url;

#[derive(Clone, Debug)]
pub struct Client<TE, TR, TT, TIR, RT, TRE>
    where
        TE: ErrorResponse,
        TR: TokenResponse<TT>,
        TT: TokenType,
        TIR: TokenIntrospectionResponse<TT>,
        RT: RevocableToken,
        TRE: ErrorResponse,
{
    client_id: ClientId,
    client_secret: Option<ClientSecret>,
    auth_url: AuthUrl,
    auth_type: AuthType,
    token_url: Option<TokenUrl>,
    redirect_url: Option<RedirectUrl>,
    introspection_url: Option<IntrospectionUrl>,
    revocation_url: Option<RevocationUrl>,
    device_authorization_url: Option<DeviceAuthorizationUrl>,
    phantom: PhantomData<(TE, TR, TT, TIR, RT, TRE)>,
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
//
// #[derive(Debug)]
// pub struct YahooClient {
//     pub request: HttpRequest
// }
// pub struct YahooConnector {
//     pub client: Client,
//     pub url: &'static str,
//     pub search_url: &'static str,
// }
//
// impl YahooConnector {
//     pub fn new() -> YahooConnector {
//         let connector = YahooConnector {
//             client: Client::new(
//                 ClientId::new(env!("YAHOO_CLIENT_ID").to_string().to_string()),
//                 Some(ClientSecret::new( env!("YAHOO_CLIENT_SECRET").to_string())),
//                 AuthUrl::new(env!("YAHOO_TOKEN_URL").to_string()).unwrap(),
//                 Some(TokenUrl::new(env!("YAHOO_TOKEN_URL").to_string()).unwrap())
//             ),
//             url: env!("").to_string().borrow(),
//             search_url: env!("").to_string().borrow(),
//         };
//
//         connector
//     }
// }
//
// impl YahooClient {
//     pub fn new() -> YahooClient {
//         YahooClient {
//             request: HttpRequest {
//                 url: Url::from_str("".to_string()),
//                 method: Method::POST,
//                 headers: Default::default(),
//                 body: vec![],
//             },
//         }
//     }
// }
//
//
// pub fn authorize(secret: String) => () {
//
// let urlencoded_id: String =
// form_urlencoded::byte_serialize( &client_id.as_bytes()).collect();
// let urlencoded_secret: String =
// form_urlencoded::byte_serialize(secret.secret().as_bytes()).collect();
// let b64_credential =
// base64::encode( & format ! ("{}:{}", & urlencoded_id, urlencoded_secret));
// headers.append( AUTHORIZATION, HeaderValue::from_str( & format ! ("Basic {}", & b64_credential)).unwrap(),
// );
//
// }

// #[derive(Default)]
// pub struct YahooConnectorBuilder {
//     inner: BasicClient,
// }

// impl YahooConnector {
//     // pub fn new(
//     //
//     // )
// }
//
// impl YahooConnector {
//     pub fn new() -> YahooConnector {
//         YahooConnector {
//             client: Client::default(),
//             url: env!["YLEAGUE_URL"],
//             search_url: env!["YSEARCH_URL"],
//         }
//     }
//
//     pub async fn get_latest_roster( &self) -> Result<YResponse, YahooError> {
//         self.get_latest_roster().await
//     }
//
// }
//
// impl YahooConnectorBuilder {
//     pub fn build(self) -> Result<YahooConnector, YahooError> {
//         let builder = Client::builder();
//
//         Ok(YahooConnector {
//             client: builder.build()?,
//             url: env!["YLEAGUE_URL"],
//             search_url: env!["YSEARCH_URL"],
//         })
//     }
//
//     pub fn timeout(mut self, timeout: Duration) -> Self {
//         self.inner = self.inner.timeout(timeout);
//
//         self
//     }
// }
//
// pub struct YResponse {
//     roster: Roster
// }
//








