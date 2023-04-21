use rand::Rng;

// Function to generate random numbers for divide
pub fn random_integer_divide(x_min : u32, x_max : u32) -> Vec<u32> {
    // create a number within [sum_min,sum_max]
    let d_min: u32 = if x_min > 1 {
        x_min
    }
    else {
        1
    };
    let num1: u32 = rand::thread_rng().gen_range(d_min..(x_max+1));

    // create a number within [0,x_max-num1]
    let num2: u32 = rand::thread_rng().gen_range(x_min..(x_max+1));

    vec![num1*num2, num1, num2]
}

// Function to generate random numbers for multiply
pub fn random_integer_multiply(x_min : u32, x_max : u32) -> Vec<u32> {
    // create a number within [sum_min,sum_max]
    let num1: u32 = rand::thread_rng().gen_range(x_min..(x_max+1));

    // create a number within [0,x_max-num1]
    let num2: u32 = rand::thread_rng().gen_range(x_min..(x_max+1));

    vec![num1, num2, num1 * num2]
}

// Function to generate random numbers for plus
pub fn random_integer_plus(sum_min : u32, sum_max : u32) -> Vec<u32> {
    // create a number within [sum_min,sum_max]
    let num1: u32 = rand::thread_rng().gen_range(sum_min..(sum_max+1));

    // create a number within [0,x_max-num1]
    let num2: u32 = rand::thread_rng().gen_range(0..(sum_max-num1+1));

    vec![num1, num2, num1 + num2]
}

// Function to generate random numbers for subtract
pub fn random_integer_subtract(sum_min : u32, sum_max : u32) -> Vec<u32> {
    // create a number within [sum_min,sum_max]
    let num1: u32 = rand::thread_rng().gen_range(sum_min..(sum_max+1));

    // create a number within [0,x_max-num1]
    let num2: u32 = rand::thread_rng().gen_range(0..(num1+1));

    vec![num1, num2, num1 - num2]
}
