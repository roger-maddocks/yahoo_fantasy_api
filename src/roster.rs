// DELETE THIS ALL HEHE oo

// use reqwest::blocking::Client;
// use serde::Deserialize;
// use reqwest::header::AUTHORIZATION;
// use http_auth_basic::Credentials;
// use reqwest::header::{Authorization, Basic};
// use std::io::Read;
// use mini_redis::client;
use reqwest::Error;
use reqwest::RequestBuilder;

#[tokio::main]
async fn main () -> Result<(), Error> {

    let builtRequest = reqwest::Client::new();
    let mut buildRequestResponse = builtRequest.get("https://api.mysportsfeeds.com/v2.1/pull/nhl/2022-2023-regular/date/20221113/games.json")
        .basic_auth("163db9fd-bdbe-4bd4-a4b8-15324f", Some("MYSPORTSFEEDS"))
        .send()
        .await
        .unwrap()
        .text()
        .await;

    println!("{:?}", buildRequestResponse);

    // let mut buildRequestResponse = builtRequest
    //     .get("https://api.mysportsfeeds.com/v2.1/pull/nhl/2022-2023-regular/date/20221113/games.json")
    //     .header("Authorization","Basic ".to_owned() + "163db9fd-bdbe-4bd4-a4b8-15324f:MYSPORTSFEEDS")
    //     .send()
    //     .await
    //     .unwrap()
    //     .text()
    //     .await;

    // let mut body = String::new();
    //
    // buildRequestResponse.read_to_string(&mut body);
    //
    // println!("Status: {}", buildRequestResponse.status());
    // println!("Headers:\n{:#?}", buildRequestResponse.headers());
    // println!("Body:\n{}", body);

    // // // //

    // let client = reqwest::blocking::Client::new();
    // let mut response = reqwest::blocking::get("https://api.mysportsfeeds.com/v2.1/pull/nhl/2022-2023-regular/date/20221113/games.json")
    //     .basic_auth("163db9fd-bdbe-4bd4-a4b8-15324f", "MYSPORTSFEED")?;
    // let mut body = String::new();
    // response.read_to_string(&mut body);
    //
    // println!("Status: {}", response.status());
    // println!("Headers:\n{:#?}", response.headers());
    // println!("Body:\n{}", body);
    //
    //
    Ok(())
}

// pub fn failedCall()



// [cfg(feature = "blocking)]
// let user_name = "163db9fd-bdbe-4bd4-a4b8-15324f".to_string();
// let credentials = Credentials::new("163db9fd-bdbe-4bd4-a4b8-15324f", "MYSPORTSFEEDS");
//
// #[tokio::main]
// async fn main() -> Result<(), Error> {
//
// //open connection to mini-redis address.
//     let mut client = client::connect("127.0.0.1:6379").await?;
//
// //set client key/value
//     client.set("hello", "world".into()).await?;
//
// //get key
//     let result = client.get("hello").await?;
//
//     println!("Value from server | result={:?}", result);
//
//     Ok(())
// }

// fn main() -> Result<(), Error> {
//     let client = reqwest::blocking::Client::new();
//
//     let user_name = "163db9fd-bdbe-4bd4-a4b8-15324f".to_string();
//     let password: Option<String> = Some("MYSPORTSFEEDS".to_string());
//
//     let response = client
//         .get("https://api.mysportsfeeds.com/v2.1/pull/nhl/2022-2023-regular/date/20221113/games.json")
//         .basic_auth(user_name, password)
//         .send();
//
//     println!("{:#?}", response);
//
//     Ok(())
// }
























// use crate::shapes::Rectangle;
// use mini_redis::{client, Result};

// #[tokio::main]
// async fn main() {
// let result = reqwest::get("https://api.spotify.com/v1/search").await;
// println!("{:?}", result)

// }


// mod shapes{
//     pub struct Rectangle {
//         pub height: i32,
//         pub width: i32,
//     }
//
//     impl Rectangle {
//         pub fn new(height: i32, width: i32) -> Rectangle {
//             Rectangle {height, width }
//         }
//         pub fn area(&self) -> i32 {
//             self.width * self.height
//         }
//     }
// }
//
// fn main() {
//
//     // pub fn area(area_of: Rectangle) -> i32 {
//     //     let total_area = area_of.width * area_of.height;
//     //     total_area
//     // }
//
//     let square = shapes::Rectangle::new(4, 4);
//
//     println!(
//         "Area of this Rectangle is {:?} in total", square.area()
//     );
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//
// //open connection to mini-redis address.
// let mut client = client::connect("127.0.0.1:6379").await?;
//
// //set client key/value
// client.set("hello", "world".into()).await?;
//
// //get key
// let result = client.get("hello").await?;
//
// println!("Value from server | result={:?}", result);
//
// Ok(())
// }


