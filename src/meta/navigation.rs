use std::io::{self, Write};

use colored::Colorize;

use crate::math::quiz::{divide_quiz, multiply_quiz, plus_quiz, subtract_quiz};
use crate::math::setting::set_difficulty;

// Function to show navigation menu
pub fn show_navigation() {
    println!("{}", "\nCurrently, Involution supports the following learning modules:".bold());
    println!("{}", "    1. Math - Quiz - Plus".bold());
    println!("{}", "    2. Math - Quiz - Subtract".bold());
    println!("{}", "    3. Math - Quiz - Multiply".bold());
    println!("{}", "    4. Math - Quiz - Divide".bold());
    println!("{}", "    S. Set Difficulty".bold());
    println!("{}", "    Q. Quit".bold());
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
        "1" => {
            let setting: Vec<u32> = set_difficulty(1);
            plus_quiz(setting[0], setting[1], setting[2]);
            false
        },
        "2" => {
            let setting: Vec<u32> = set_difficulty(2);
            subtract_quiz(setting[0], setting[1], setting[2]);
            false
        },
        "3" => {
            let setting: Vec<u32> = set_difficulty(3);
            multiply_quiz(setting[0], setting[1], setting[2]);
            false
        },
        "4" => {
            let setting: Vec<u32> = set_difficulty(4);
            divide_quiz(setting[0], setting[1], setting[2]);
            false
        },
        "Q" => {true},
        "q" => {true},
        _ => {println!("{}", "Option is not supported!".red()); false},
    };

    breaking
}
