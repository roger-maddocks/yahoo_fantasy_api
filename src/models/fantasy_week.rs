use crate::player::Position;
use chrono::Days;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FantasyWeek {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

impl FantasyWeek {
    pub fn new(beginning_week: u64, ending_week: u64) -> FantasyWeek {
        FantasyWeek {
            start: NaiveDate::from_ymd_opt(2023, 10, 2)
                .unwrap()
                .checked_add_days(Days::new(beginning_week * 7))
                .unwrap(),
            end: NaiveDate::from_ymd_opt(2023, 10, 8)
                .unwrap()
                .checked_add_days(Days::new(ending_week * 7))
                .unwrap(),
        }
    }

    pub fn get_season_start_week(&self) -> FantasyWeek {
        FantasyWeek::new(1, 1)
    }

    pub fn get_season_end_week(&self) -> FantasyWeek {
        FantasyWeek::new(26, 26)
    }

    pub fn get_week(&self, week_number: u64) -> FantasyWeek {
        FantasyWeek::new(week_number, week_number)
    }

    //can be changed to array due to known size? Any benefit?
    pub fn get_week_range(
        &self,
        first_week_of_range: u64,
        last_week_of_range: u64,
    ) -> Vec<FantasyWeek> {
        let mut all_weeks = vec![];
        for week in first_week_of_range..last_week_of_range + 1 {
            all_weeks.push(FantasyWeek::new(week, week))
        }
        all_weeks
    }
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct FantasySchedule {
// }
