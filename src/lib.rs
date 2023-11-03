use std::fmt::Error;
use reqwest::blocking::{Client, ClientBuilder};
const YCHART_URL: &str = "https://query1.finance.yahoo.com/v8/finance/chart";
const YSEARCH_URL: &str = "https://query2.finance.yahoo.com/v1/finance/search";

macro_rules! fantasy_games_by_day {
    () => {"{url}"};
}

pub struct YahooFantasyConnector {
    client: Client,
    url: &'static str,
    search_url: &'static str
}

#[derive(Default)]
pub struct YahooConnectorBuilder {
    inner: ClientBuilder,
}


impl YahooConnectorBuilder {
    pub fn build(self) -> Result<YahooFantasyConnector, yahoo_fantasy::YahooError> {
        let fc = YahooFantasyConnector::new();
        Ok((fc))
    }

}
