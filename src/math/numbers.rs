use rand::Rng;

// Function to generate random numbers for plus math
pub fn random_integer_plus(sum_min : u32, sum_max : u32) -> Vec<u32> {
    // create a number within [sum_min,sum_max]
    let num1: u32 = rand::thread_rng().gen_range(sum_min..(sum_max+1));

    // create a number within [0,x_max-num1]
    let num2: u32 = rand::thread_rng().gen_range(0..(sum_max-num1+1));

    vec![num1, num2, num1 + num2]
}
