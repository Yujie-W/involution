use std::io::{self, Write};

use colored::Colorize;

use super::super::math::quiz::{divide_quiz, multiply_quiz, plus_quiz, subtract_quiz};

// Function to show navigation menu
pub fn show_navigation() {
    println!("{}", "\nCurrently, Involution supports the following learning modules:".bold());
    println!("{}", "    1. Math - Quiz - Plus".bold());
    println!("{}", "    2. Math - Quiz - Subtract".bold());
    println!("{}", "    3. Math - Quiz - Multiply".bold());
    println!("{}", "    4. Math - Quiz - Divide".bold());
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
        "1" => {plus_quiz(5, 0, 20); false},
        "2" => {subtract_quiz(5, 0, 20); false},
        "3" => {multiply_quiz(5, 0, 10); false},
        "4" => {divide_quiz(5, 0, 10); false},
        "Q" => {true},
        "q" => {true},
        _ => {println!("{}", "Option is not supported!".red()); false},
    };

    breaking
}
