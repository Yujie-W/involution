use super::format::{display_question, display_result};
use super::input::get_input_number;
use super::numbers::{random_integer_divide, random_integer_multiply, random_integer_plus, random_integer_subtract};

// Function for plus quiz in the main menu
pub fn divide_quiz(n : u32, x_min : u32, x_max : u32) {
    println!("\nMath multiply quiz with {n} questions!");

    // Generate n questions
    for i in 0..n {
        let nums: Vec<u32> = random_integer_divide(x_min,x_max);
        let mesg: String = display_question(i, &nums, "/".to_string());
        let user_answ : u32 = get_input_number(mesg);
        display_result(nums[2], user_answ);
    }
}

// Function for plus quiz in the main menu
pub fn multiply_quiz(n : u32, x_min : u32, x_max : u32) {
    println!("\nMath multiply quiz with {n} questions!");

    // Generate n questions
    for i in 0..n {
        let nums: Vec<u32> = random_integer_multiply(x_min,x_max);
        let mesg: String = display_question(i, &nums, "x".to_string());
        let user_answ : u32 = get_input_number(mesg);
        display_result(nums[2], user_answ);
    }
}

// Function for plus quiz in the main menu
pub fn plus_quiz(n : u32, x_min : u32, x_max : u32) {
    println!("\nMath plus quiz with {n} questions!");

    // Generate n questions
    for i in 0..n {
        let nums: Vec<u32> = random_integer_plus(x_min,x_max);
        let mesg: String = display_question(i, &nums, "+".to_string());
        let user_answ : u32 = get_input_number(mesg);
        display_result(nums[2], user_answ);
    }
}

// Function for subtract quiz in the main menu
pub fn subtract_quiz(n : u32, x_min : u32, x_max : u32) {
    println!("\nMath subtract quiz with {n} questions!");

    // Generate n questions
    for i in 0..n {
        let nums: Vec<u32> = random_integer_subtract(x_min,x_max);
        let mesg: String = display_question(i, &nums, "-".to_string());
        let user_answ : u32 = get_input_number(mesg);
        display_result(nums[2], user_answ);
    }
}
