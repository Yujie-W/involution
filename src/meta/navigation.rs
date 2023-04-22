use colored::Colorize;

use crate::math::input::get_input_number;
use crate::math::quiz::{divide_quiz, multiply_quiz, plus_quiz, subtract_quiz};
use crate::math::setting::set_difficulty;

// Function to show navigation menu
pub fn show_navigation() {
    println!("{}", "\nCurrently, Involution supports the following learning modules:".bold());
    println!("{}", "    1. Math - Quiz - Plus".bold());
    println!("{}", "    2. Math - Quiz - Subtract".bold());
    println!("{}", "    3. Math - Quiz - Multiply".bold());
    println!("{}", "    4. Math - Quiz - Divide".bold());
    println!("{}", "    0. Quit".bold());
}

// Function to run functions based on user inputs
pub fn choose_module() -> bool {
    // Read user inputs
    let input_number: u32 = get_input_number("Please indicate your choice > ".to_string());

    // Match user inputs to existing functions
    let breaking: bool = match input_number {
        1 => {
            let setting: Vec<u32> = set_difficulty(1);
            plus_quiz(setting[0], setting[1], setting[2]);
            false
        },
        2 => {
            let setting: Vec<u32> = set_difficulty(2);
            subtract_quiz(setting[0], setting[1], setting[2]);
            false
        },
        3 => {
            let setting: Vec<u32> = set_difficulty(3);
            multiply_quiz(setting[0], setting[1], setting[2]);
            false
        },
        4 => {
            let setting: Vec<u32> = set_difficulty(4);
            divide_quiz(setting[0], setting[1], setting[2]);
            false
        },
        0 => {true},
        _ => {println!("{}", "Option is not supported!".red()); false},
    };

    breaking
}
