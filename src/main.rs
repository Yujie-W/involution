use colored::Colorize;

mod math;
mod meta;

fn main() {
    // display version number of the
    meta::version::show_version_info();

    // show the navigation in a while loop
    let mut counter: u32 = 0;
    while counter < 9999 {
        counter += 1;
        meta::navigation::show_navigation();
        let choice: String = meta::navigation::choose_module();
        let matchx: &str = choice.trim();
        let breaking: bool = match matchx {
            "1" => {math::plus::quiz(50, 0, 20); false},
            "Q" => {true},
            "q" => {true},
            _ => {println!("{}", "Option is not supported!".red()); false},
        };
        if breaking {
            break;
        }
    }

}
