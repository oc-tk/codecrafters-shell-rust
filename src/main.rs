#[allow(unused_imports)]
use std::io::{self, Write};



fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let input_formatted = input.strip_suffix("\r\n")
         .or(input.strip_suffix("\n"))
         .unwrap();

    print!("{input_formatted}: command not found");
}
