use crate::models::report::Report;

pub fn format_collision_report_block(this_indexed_report: &mut (&u64, &Report)) {
    println!(" ------------------");
    println!("|     WEEK {:?}      |", this_indexed_report.0.clone());
    println!("| COLLISION REPORT |");
    println!(" ------------------");
}

pub fn print_starting_block(week: u64) {
    println!();
    format_week_starting_block_separator(week);
    println!("|--------- WEEK {} ---------|", week);
    format_week_starting_block_separator(week);
    println!();
    println!("------------------------------------");
    println!("|   Teams with 4 Games this week   |");
    println!("------------------------------------");
}


pub fn format_based_on_description(description: &str) {
    match description {
        x if x.contains("front") => {
            println!("---------------------------------------------------------------")
        }
        x if x.contains("back") => {
            println!("--------------------------------------------------------------")
        }
        _ => panic!("Error formatting front/back description"),
    }
}
fn format_week_starting_block_separator(week: u64) {
    match week {
        x if x < 10 => {
            println!("|--------------------------|")
        }
        x if x >= 10 => {
            println!("|---------------------------|")
        }
        _ => panic!("Error formatting week print"),
    }
}
