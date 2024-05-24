#[allow(unused_imports)]
use std::io::{self, Write};



fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    input.strip_suffix("\r\n")
         .or(input.strip_suffix("\n"))
         .unwrap_or(input)

    print!("{inputFormatted}: command not found");
}
