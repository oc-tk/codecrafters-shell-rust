#[allow(unused_imports)]
use std::io::{self, Write};



fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    //stdin.read_line(&mut input).unwrap();

    let lines = io::stdin().lines();
    for line in lines {
        let command = input.trim();
        println!("{command}: command not found");
    }

    let command = input.trim();
    println!("{command}: command not found");
}
