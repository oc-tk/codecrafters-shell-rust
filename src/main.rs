
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
            x  if x.to_string().contains("exit") => {
                if let Some(last_char) = command.chars().last() {
                    match last_char.to_string().parse::<i32>() {
                        Ok(status) => process::exit(status),
                        Err(e) => println!("Failed to parse the status code: {}", e),
                    }
                }
            },
            _ => println!("{command}: command not found"),
        }
    }
}
