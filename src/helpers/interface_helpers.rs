use crate::helpers::visual_helpers::print_program_options;
use std::io::stdin;
pub fn user_requests_free_agents(user_command: &str) -> bool {
    if user_command == "fa" {
        true
    } else if user_command == "free agents" {
        true
    } else if user_command == "freeagents" {
        true
    } else if user_command == "freeagent" {
        true
    } else if user_command == "free agent" {
        true
    } else {
        false
    }
}

pub fn exit_program(input: &str) -> bool {
    match input {
        x if x == "q" => true,
        x if x == "quit" => true,
        _ => false,
    }
}

pub fn user_requests_weekly_reports(user_command: &str) -> bool {
    if user_command == "wr" {
        true
    } else if user_command == "weekly reports" {
        true
    } else if user_command == "weeklyreports" {
        true
    } else if user_command == "weekreport" {
        true
    } else if user_command == "week report" {
        true
    } else if user_command == "weekly report" {
        true
    } else {
        false
    }
}

pub fn get_user_command() -> String {
    print_program_options();
    let mut input = String::new();
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
