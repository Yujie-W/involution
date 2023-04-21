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
