
use std::process;
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
    
        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let command = input.trim();

        match command {
            x  if x.to_string().contains("exit") => process::exit(0),
            _ => println!("{command}: command not found"),
        }

        println!("{command}: command not found"); 
    }
}
