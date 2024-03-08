use crate::models::report::Report;

pub fn print_program_options() {
    println!("Enter \"S\" for initial setup.");
    println!("Enter \"SC\" for tracked stat categories.");
    println!("Enter \"FA\" for Free Agency operations.");
    println!("Enter \"WR\" to view Week Report from specific week.");
    println!("Enter \"TW\" to view Week Report for This Week.");
    println!("Enter \"NW\" to view Week Report for Next Week.");
    println!("Enter \"CR\" to view Collision Report from specific week.");
    println!("Enter \"CD\" to view Commissioner Dashboard from specific week.");
    println!("Enter \"Q\" to quit.");
}

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

pub fn not_implemented(x: &str) {
    println!(
        "Whoops, I misguided you. The {:?} functionality is not supported yet!",
        x
    );
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
