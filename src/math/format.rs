use colored::Colorize;

// Function to format the numbers to strings to display in terminal
pub fn rendered_numbers(nums : &Vec<u32>) -> Vec<String> {
    let mut max_len: usize = 4;
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

// Function to display the question in terminal
pub fn display_question(quest_ind : u32, nums : &Vec<u32>, oper : String) {
    let ques: String = format!("Question {quest_ind}: What is the result of {} {oper} {}?", nums[0], nums[1]);
    println!("{}", ques.blue());

    let strs: Vec<String> = rendered_numbers(&nums);

    println!("{}", strs[0]);
    println!("{oper}{}", strs[1]);
    println!("{}", "-".repeat(strs[0].len()+1));
    print!("{}", "= ".yellow());
}
