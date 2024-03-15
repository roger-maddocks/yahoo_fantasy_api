use crate::helpers::visual_helpers::print_program_options;
use std::io::stdin;

pub fn user_requests_free_agents(user_command: &str) -> bool {
    user_command == "fa"
        || user_command == "free agents"
        || user_command == "freeagents"
        || user_command == "freeagent"
        || user_command == "free agent"
}

pub fn exit_program(input: &str) -> bool {
    input == "q" || input == "quit"
}

pub fn user_requests_weekly_reports(user_command: &str) -> bool {
    user_command == "wr"
        || user_command == "weekly reports"
        || user_command == "weeklyreports"
        || user_command == "weekreport"
        || user_command == "week report"
        || user_command == "weekly report"
}

pub fn get_user_command() -> String {
    let mut input = String::new();

    print_program_options();
    stdin()
        .read_line(&mut input)
        .expect("Issue reading User input!");
    input.to_lowercase().trim().parse().unwrap()
}
pub fn get_user_report_bounds() -> u64 {
    println!("Enter which week (1 - 26) you would like the report for.");

    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Issue reading User input!");

    input
        .trim()
        .to_lowercase()
        .parse()
        .expect("User did not enter numeric value!")
}
