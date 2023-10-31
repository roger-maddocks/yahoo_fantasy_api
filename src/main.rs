use reqwest::Error;
use crate::fantasy_week::FantasyWeek;
use crate::team::Team;
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

#[tokio::main]
async fn main () -> Result<(), Error> {
    for i in 4 ..= 4  {
        let this_week = FantasyWeek::new(i, i);
        weekly_data_factory::get_loaded_schedule_report(i, &this_week).await;
    }

    Ok(())
}
