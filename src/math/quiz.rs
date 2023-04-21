use super::format::{display_question, display_result};
use super::input::get_input_number;
use super::numbers::random_integer_plus;

// Function for plus quiz in the main menu
pub fn plus_quiz(n : u32, x_min : u32, x_max : u32) {
    println!("\nMath quiz with {n} questions!\n");

    // Generate n questions
    for i in 0..n {
        let nums: Vec<u32> = random_integer_plus(x_min,x_max);
        display_question(i, &nums, "+".to_string());
        let user_answ : u32 = get_input_number();
        display_result(nums[2], user_answ);
        println!("\n");
    }
}
