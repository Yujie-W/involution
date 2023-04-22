use std::io;

// Function to get an integer input from terminal
pub fn get_input_number() -> u32 {
    let mut input_number : u32 = 0;
    let mut breaking : bool = false;

    while !breaking {
        let mut input_text: String = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed: &str = input_text.trim();
        match trimmed.parse::<u32>() {
            Ok(converted ) => {
                input_number = converted;
                breaking = true;
            }
            Err(..) => {
                println!("The input {input_text} was not an integer, please try again!");
            }
        }
    };

    input_number
}
