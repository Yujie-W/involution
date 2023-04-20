use std::io::{self, Write};

use colored::Colorize;
use rand::Rng;

fn random_unit(x_min : u32, x_max : u32) -> Vec<u32> {
    let num1: u32 = rand::thread_rng().gen_range(x_min..x_max);
    let num2: u32 = rand::thread_rng().gen_range(x_min..x_max);

    vec![num1, num2]
}

fn rendered_numbers(nums : &Vec<u32>) -> Vec<String> {
    let sum : u32 = nums.iter().sum();
    let mut max_len: usize = sum.to_string().len() + 2;
    for num in nums.iter() {
        if num.to_string().len() + 2 > max_len {
            max_len = num.to_string().len() + 2;
        }
    }

    let mut strs : Vec<String> = vec![];
    for i in 0..nums.len() {
        let x_len: usize = nums[i].to_string().len();
        let s_len = if i == 0 {
            max_len - x_len
        }
        else {
            max_len - x_len - 1
        };
        let x_str: String = format!("{}{}"," ".repeat(s_len), nums[i].to_string());
        strs.push(x_str);
    }

    strs
}

pub fn quiz(n : u32) {
    println!("\nMath quiz with {n} questions!\n");
    for i in 0..n {
        let nums: Vec<u32> = random_unit(0,100);
        let ques: String = format!("Question {i}: What is the sum of {} + {}?", nums[0], nums[1]);
        println!("{}", ques.yellow());
        let answ: u32 = nums.iter().sum();
        let strs: Vec<String> = rendered_numbers(&nums);
        println!("{}", strs[0]);
        println!("+{}", strs[1]);
        println!("{}", "-".repeat(strs[0].len()+1));
        print!("= ");
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
