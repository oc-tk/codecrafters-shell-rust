
use std::process;
use std::io::{self, Write};

fn handle_exit_command(command: &str) {
    //lets fancly do it
    if let Some(last_char) = command.chars().last() {
        match last_char.to_string().parse::<i32>() {
            Ok(status) => process::exit(status),
            Err(e) => println!("Failed to parse the status code: {}", e),
        }
    } else {
        println!("Failed to get last char of exit command (missing status code).");
    }
}

fn handle_echo_command(command: &str) {
    let echo_less_command = command.trim().replace("echo ", "");
    print!("{echo_less_command}");
}

//handle pattern matching
fn handle_matching(command: &str) {
    match command {
        x  if x.to_string().contains("exit") => handle_exit_command(&command), //guard matching any "exit x" command where x stands for status code
        x  if x.to_string().contains("echo") => handle_echo_command(&command), //guard matching any "exit x" command where x stands for status code
        _ => println!("{command}: command not found"), //default case where command is not implemented
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
    
        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        handle_matching(input.trim());
    }
}
