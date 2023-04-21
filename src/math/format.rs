use std::io::{self, Write};

use colored::Colorize;

// Function to format the numbers to strings to display in terminal
fn rendered_numbers(nums : &Vec<u32>) -> Vec<String> {
    // Default length is 4
    let mut max_len: usize = 4;

    // Iterate through the numbers and change the length accordingly (max digits + 2)
    for num in nums.iter() {
        if num.to_string().len() + 2 > max_len {
            max_len = num.to_string().len() + 2;
        }
    }

    // Create an empty vector of string, and add leading spaces
    let mut strs : Vec<String> = vec![];
    for i in 0..nums.len() {
        let x_len: usize = nums[i].to_string().len();
        // Add (N - l) spaces for first element
        let s_len: usize = if i == 0 {
            max_len - x_len
        }
        // Add (N -l -1) spaces for second element (1 for operator such as +)
        else {
            max_len - x_len - 1
        };
        let x_str: String = format!("{}{}", " ".repeat(s_len), nums[i].to_string());
        strs.push(x_str);
    }

    strs
}

// Function to display the question in terminal
pub fn display_question(quest_ind : u32, nums : &Vec<u32>, oper : String) {
    // Display the question
    let ques: String = format!("Question {quest_ind}: What is the result of {} {oper} {}?", nums[0], nums[1]);
    println!("{}", ques.blue());

    // Format the numbers and display the question vertically
    let strs: Vec<String> = rendered_numbers(&nums);
    println!("{}", strs[0]);
    println!("{oper}{}", strs[1]);
    println!("{}", "-".repeat(strs[0].len()+1));
    print!("{}", "= ".yellow());
    io::stdout()
        .flush()
        .expect("failed to flush");
}

// Function to display result comparison
pub fn display_result(answer : u32, input : u32) {
    // Show message in green if the answer is correct
    if input == answer {
        let mesg: String = format!("You were correct, the answer is {}", answer);
        println!("{}", mesg.green().bold());
    }
    // Show message in red if the answer is incorrect
    else {
        let mesg: String = format!("Oh no, you were wrong! The correct answer is {}", answer);
        println!("{}", mesg.red().bold());
    }
    println!("\n");
}
