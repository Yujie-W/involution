use std::io::{self, Write};

use colored::Colorize;

use super::format::{display_question};
use super::numbers::random_integer_plus;

// Function for plus quiz in the main menu
pub fn quiz(n : u32, x_min : u32, x_max : u32) {
    println!("\nMath quiz with {n} questions!\n");
    for i in 0..n {
        let nums: Vec<u32> = random_integer_plus(x_min,x_max);

        display_question(i, &nums, "+".to_string());

        io::stdout()
            .flush()
            .expect("failed to flush");

        let mut input_text: String = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();
        match trimmed.parse::<u32>() {
            Ok(user_answ ) => {
                if user_answ == nums[2] {
                    let mesg: String = format!("You were correct, the answer is {}", nums[2]);
                    println!("{}", mesg.green().bold());
                }
                else {
                    let mesg: String = format!("Oh no, you were wrong! The correct answer is {}", nums[2]);
                    println!("{}", mesg.red().bold());
                }
            }
            Err(..) => println!("this was not an integer: {}", trimmed),
        };
        println!("\n");
    }
}
