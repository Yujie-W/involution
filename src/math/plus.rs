use std::io;

use super::format::{display_question, display_result};
use super::numbers::random_integer_plus;

// Function for plus quiz in the main menu
pub fn quiz(n : u32, x_min : u32, x_max : u32) {
    println!("\nMath quiz with {n} questions!\n");
    for i in 0..n {
        let nums: Vec<u32> = random_integer_plus(x_min,x_max);

        display_question(i, &nums, "+".to_string());

        let mut input_text: String = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();
        match trimmed.parse::<u32>() {
            Ok(user_answ ) => {
                display_result(nums[2], user_answ);
            }
            Err(..) => println!("this was not an integer: {}", trimmed),
        };
        println!("\n");
    }
}
