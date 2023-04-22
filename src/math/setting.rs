use std::io::{self, Write};

use super::input::get_input_number;

// Function to set up difficulty
pub fn set_difficulty(quiz_type : u8) -> Vec<u32> {
    // Display the notes for the settings
    println!("\nThe settings include:");
    println!("    (a) Number of questions");
    println!("    (b) Minimum number");
    println!("    (c) Maximum number");
    println!("The default number of question is 20.");
    let defaults: Vec<u32> = match quiz_type {
        1 => {
            println!("z of x + y = z is within [min,max], and the default is [0,20].");
            vec![20, 0, 20]
        },
        2 => {
            println!("x of x - y = z is within [min,max], and the default is [0,20].");
            vec![20, 0, 20]
        },
        3 => {
            println!("x and y of x * y = z are within [min,max], and the default is [0,10].");
            vec![20, 0, 10]
        },
        4 => {
            println!("x and y of x * y = z are within [min,max], and the default is [0,10].");
            vec![20, 0, 10]
        },
        _ => {
            vec![0, 0, 0]
        }
    };

    // Press RETURN to skip the setting
    print!("\nIf you want to customize the difficulty, type S; otherwise, press ENTER to continue > ");
    io::stdout()
        .flush()
        .expect("failed to flush");

    let mut input_text: String = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    let trimmed: &str = input_text.trim();
    match trimmed {
        "S" => {},
        "s" => {},
        _ => {return defaults}
    }

    // Ask for inputs
    print!("\nPlease input the number of questions in a quiz > ");
    io::stdout()
        .flush()
        .expect("failed to flush");
    let n: u32 = get_input_number();

    print!("Please input the minimum number of the quiz > ");
    io::stdout()
        .flush()
        .expect("failed to flush");
    let x_min: u32 = get_input_number();

    print!("Please input the maximum number of the quiz > ");
    io::stdout()
        .flush()
        .expect("failed to flush");
    let x_max: u32 = get_input_number();

    return vec![n, x_min, x_max]
}
