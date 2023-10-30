// use std::borrow::Borrow;
// use std::vec::IntoIter;
// use chrono::NaiveDate;
// use chrono::DateTime;
// use chrono::Days;
// use serde::{Deserialize, Serialize};
// use crate::fantasy_week::FantasyWeek;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct FantasySchedule {
// }
//
// impl FantasySchedule {
//     pub fn get_start_week(&self) -> FantasyWeek {
//         FantasyWeek::new( &self, 1, 1)
//     }
//
//     pub fn get_week(&self, week_number: u64) -> FantasyWeek {
//         FantasyWeek::new( &self, week_number, week_number)
//     }
//
//     pub fn get_week_range(&self, first_week_of_range: u64, last_week_of_range: u64) -> FantasyWeek {
//         FantasyWeek::new( &self, first_week_of_range, last_week_of_range)
//     }
// }
//

// TODO format the weird all star week
// pub fn get_all_star_week(&self) -> FantasyWeek {
// return FantasyWeek {
//     start: (),
//     end: ()
// }
// }
// TODO

// impl IntoIterator for FantasyWeek {
//     type Item = NaiveDate;
//     type IntoIter = FantasyWeekIntoIterator;
//
//     fn into_iter(self) -> Self::IntoIter {
//         FantasyWeekIntoIterator {
//             fantasy_week: self,
//             index:  0
//         }
//     }
// }

// pub struct FantasyWeekIntoIterator {
//     fantasy_week: FantasyWeek,
//     index: u64
// }












// impl<'a> IntoIterator for &'a FantasyWeek {
//     type Item = NaiveDate;
//     type IntoIter = FantasyWeekIterator<'a>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         FantasyWeekIterator {
//             fantasy_week: self,
//             index: 0
//         }
//     }
// }
//
// struct FantasyWeekIterator<'a> {
//     fantasy_week: &'a FantasyWeek,
//     index: u64
// }
//
// impl<'a> Iterator for FantasyWeekIterator<'a> {
//     type Item = &'a NaiveDate;
//
//     fn next(&mut self) -> Option<NaiveDate> {
//         let current_day = self.fantasy_week.start.checked_add_days(Days::new(self.index)).unwrap();
//         if current_day <= self.fantasy_week.end {
//             self.index += 1;
//             Some(current_day)
//         } else {
//             None
//         }
//     }
// }

// pub fn iter_weeks(&self) -> Iter<FantasyWeek> {
// }

// impl FantasyWeek {
//     pub fn iter_days(&self) -> Iter<String> {
//         // calculate all the days in between start and end
//         // for each day
//         // convert the day to the right datetimefmt
//         // return iter of it
//     }
// }

// pub struct Week {
//     pub start_date: chrono::DateTime,
//     pub end_date: chrono::DateTime
// }
//
// impl Week {
//     fn new( monday: String,
//             tuesday: String,
//             wednesday: String,
//             thursday: String,
//             friday: String,
//             saturday: String,
//             sunday: String
//     ) -> Self {
//         Self {
//             monday,
//             tuesday,
//             wednesday,
//             thursday,
//             friday,
//             saturday,
//             sunday
//         }
//     }
//     //
//     // PUB FN ITER_DAYS(&SELF) -> ITER<STRING> {
//     // }
// }
//
// fn week_days() {
// }
//
//
// // impl Index<>
//
// impl IntoIterator for Week {
//     type Item = String;
//     type IntoIter = WeekIntoIterator;
//
//     fn into_iter(self) -> Self::IntoIter {
//         WeekIntoIterator {
//             week: self,
//             index: 0,
//         }
//     }
// }
//
// pub struct WeekIntoIterator {
//     week: Week,
//     index: usize,
// }
//
// impl Iterator for WeekIntoIterator {
//     type Item = String;
//
//     fn next(&mut self) -> Option<&String> {
//         let result = match self.index {
//             0 => &self.week.monday,
//             1 => &self.week.tuesday,
//             2 => &self.week.wednesday,
//             3 => &self.week.thursday,
//             4 => &self.week.friday,
//             5 => &self.week.saturday,
//             6 => &self.week.sunday,
//             _ => return None,
//         };
// //         self.index += 1;
// //         Some(result)
// //     }
// // }
//
// fn get_week(week: i32)  -> Week {
//     if week == 1 {
//         regular_season::Week {
//             monday: "20231009".to_string(),
//             tuesday: "20231010".to_string(),
//             wednesday: "20231011".to_string(),
//             thursday: "20231012".to_string(),
//             friday: "20231013".to_string(),
//             saturday: "20231014".to_string(),
//             sunday: "20231015".to_string(),
//         }
//     } else {
//         regular_season::Week {
//             monday: "".to_string(),
//             tuesday: "".to_string(),
//             wednesday: "".to_string(),
//             thursday: "".to_string(),
//             friday: "".to_string(),
//             saturday: "".to_string(),
//             sunday: "".to_string(),
//         }
//     }
// }
//
