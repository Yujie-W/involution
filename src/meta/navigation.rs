use std::io::{self, Write};

use colored::Colorize;

pub fn show_navigation() {
    println!("{}", "Currently, Involution supports the following learning modules:".bold());
    println!("{}", "    1. Math - Plus - Quiz".bold());
    println!("{}", "    Q. Save and Quit".bold());
    print!("{}", "Please indicate your choice > ".yellow());
    io::stdout()
        .flush()
        .expect("failed to flush");
}

pub fn choose_module() -> String {
    let mut input_text: String = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    println!("");

    input_text
}
