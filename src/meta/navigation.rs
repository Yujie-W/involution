use std::io::{self, Write};

use colored::Colorize;

use super::super::math::quiz::plus_quiz;

// Function to show navigation menu
pub fn show_navigation() {
    println!("{}", "Currently, Involution supports the following learning modules:".bold());
    println!("{}", "    1. Math - Plus - Quiz".bold());
    println!("{}", "    Q. Save and Quit".bold());
    print!("{}", "Please indicate your choice > ".yellow());
    io::stdout()
        .flush()
        .expect("failed to flush");
}

// Function to run functions based on user inputs
pub fn choose_module() -> bool {
    // Read user inputs
    let mut input_text: String = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    // Match user inputs to existing functions
    let choice = input_text.trim();
    let breaking = match choice {
        "1" => {plus_quiz(50, 0, 20); false},
        "Q" => {true},
        "q" => {true},
        _ => {println!("{}", "Option is not supported!".red()); false},
    };
    println!("");

    breaking
}
