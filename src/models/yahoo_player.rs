use std::fmt;
use std::fmt::Formatter;
use chrono::NaiveDate;
// use serde_xml;
use oauth2::http::HeaderMap;
use serde::Deserialize;
// use serde::Deserialize;


#[derive(Default, serde::Serialize,Deserialize,  Debug, Clone)]
pub struct YahooPlayers {
    players: Vec<YahooPlayer>
}

#[derive(Default, serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct YahooPlayer {
    player_key: String,
    player_id: u64,
    name: String,
    full: String,
    first: String,
    last: String,
    ascii_first: String,
    ascii_last: String,
    // </name>\n
    url: String,
    editorial_player_key: String,
    editorial_team_key: String,
    editorial_team_full_name: String,
    editorial_team_abbr: String,
    editorial_team_url: String,
    is_keeper: String,
    status: String,
    cost: String,
    kept: String,
    // </is_keeper>\n
    uniform_number: u64,
    display_position: String,
    headshot: Headshot,
    is_undroppable: bool,
    position_type: String,
    primary_position: String,
    eligible_positions: Vec<String>,
    eligible_positions_to_add: Vec<String>,
    has_player_notes: bool,
    player_notes_last_timestamp: u64,
    selected_position: SelectedPosition,
    is_editable: bool,
}

// impl fmt::Debug for YahooPlayer {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         f.debug_map("")
//             .key("").
//
//     }
// }


#[derive(Default, serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Headshot {
    url: String,
    size: String,
}
#[derive(Default, serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SelectedPosition {
    coverage_type: String,
    date: NaiveDate,
    is_flex: bool,
}
