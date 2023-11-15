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

    yahoo_auth_profile::get_redirect_url_for_auth_code();

    let auth = yahoo_auth_profile::YahooEncode::new();
    let client = reqwest::Client::new();
    let response = client
        .post(auth.token_url)
        .form(&auth.token_params)
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


// "{\"access_token\":\"c.hHlNOYuwtk23y1tx_M3Kq6HWqT5GvJbgtWLy_cjIn4SXkwcNgF3.9L7XjIsy1m_MJ1PAPsoF_Lt0fe4jtUtDURx.5jFVVnnKjqabVVEK7bZJia8QBySdnWI.xd6ovJAC3FQgNXrFvgyZsQI8Dank9MPJQlt_1TnX9XcCq_BaHzut5VsfwPdHPgYEGghTqbIXr_eh9bQic5RlQtWlA5UtmSTyp9jLRp7kn4dmupX_TGAwU8tl7B3WogGoUL.C_U3u_Ji1ev8vZW97gMil3i3PF4MKoOU0Us_gkBsyhclg8yaXqNr5MG0_81TS9TNxcZfG6GyfmdSLRgz.DQlhdK5oBiFqm_mQmtRm0aCA.lRA6VGvePVzK6XYT50e6Pn_afVfAXU.zQd8j0ig7BYTEVEysRUTTp8MJnO26myiyqTRq0wFY.wHrnH55AVk5vfKmcai3delD2sqfmjj87IRjkvdt1oKqHJzAb0FFiVjCbKY5pvOR.vhhZ2_667aBS_1IXeFiahbUPlGKmLH4_LkRUnKdqJtrjFRy0xvWW1S0Ljs941PfbST8Cwgrt_pBUgSJqEKGFB3H21hQHMQsv4ClQpYD4ZDAXuOQNyJ_Knq3EXxMYr3UWDeZmcUyxssXldplIEKyPMqcncnPAIn9zePSTmvO_mwewbqY1UMcl97OS_JmGwa9JOmmAT8pMk3MXRhNq34YNM54hnpsTV5drsi51LabVrgkUetk4m0VJVHgLb2nZ6trFBSjFb5m_DMCo5OTHQv479A1lLfLiNrEiym_fZc.YypE4Ymg7SmX3ur9XviUCxJszcjlwLcZewDubpgGm2aRizrOkWlhoBChrtIa4NtJw3zAnupPB59AJBQ2LzRhSosKgy3RJL_UI40Pi26k3Kov8vNV0v_JNhfD3aaKKa82qAlov8J7LTUOa5zVVGmqWPhcZR2ADqgAQ_BHFQE22kl8rLxbGqNV8XKPod_w76TlxpIK859ZgyO9V\",\"refresh_token\":\"AN9NVWXQXPxYtB9FB5wWJvhfBhhm~000~yCpm41AyJxoTlNjhjmJCmCvA1Fmj9LjgQr.E\",\"expires_in\":3600,\"token_type\":\"bearer\"}"