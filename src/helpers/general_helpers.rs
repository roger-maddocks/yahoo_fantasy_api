use std::io::stdin;
use crate::helpers::visual_helpers::print_program_options;

pub fn get_day_from_number(day: &u64) -> String {
    let mut named_day = "PANIK";
    match *day {
        0 => { named_day = "Monday"}
        1 => { named_day = "Tuesday"}
        2 => { named_day = "Wednesday"}
        3 => { named_day = "Thursday"}
        4 => { named_day = "Friday"}
        5 => { named_day = "Saturday"}
        6 => { named_day = "Sunday"}
        _ => {}
    }
    named_day.to_string()
}

