use std::io::{self, Write};

use colored::Colorize;

use super::format::rendered_numbers;
use super::numbers::random_integer_plus;

pub fn quiz(n : u32, x_min : u32, x_max : u32) {
    println!("\nMath quiz with {n} questions!\n");
    for i in 0..n {
        let nums: Vec<u32> = random_integer_plus(x_min,x_max);
        let ques: String = format!("Question {i}: What is the sum of {} + {}?", nums[0], nums[1]);
        println!("{}", ques.blue());
        let answ: u32 = nums.iter().sum();
        let strs: Vec<String> = rendered_numbers(&nums);
        println!("{}", strs[0]);
        println!("+{}", strs[1]);
        println!("{}", "-".repeat(strs[0].len()+1));
        print!("{}", "= ".yellow());
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
                if user_answ == answ {
                    let mesg: String = format!("You were correct, the answer is {}", answ);
                    println!("{}", mesg.green().bold());
                }
                else {
                    let mesg: String = format!("Oh no, you were wrong! The correct answer is {}", answ);
                    println!("{}", mesg.red().bold());
                }
            }
            Err(..) => println!("this was not an integer: {}", trimmed),
        };
        println!("\n");
    }
}
