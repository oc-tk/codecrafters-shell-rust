
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
        print!("Failed to get last char of exit command (missing status code).");
    }
}

fn handle_echo_command(command: &str) {
    let echoless_command = command.replace("echo ", "");
    println!("{echoless_command}");
}

fn handle_type_command(command: &str) {
    let typeless_command = command.trim().replace("type ", "");
    println!("{typeless_command} is a shell builtin");
}

//handle pattern matching
fn handle_matching(input: &str) {
    if let Some(command) = input.split_whitespace().next() {
        //normally interpreting first part of entry
        match command {
            x  if x.to_string().contains("exit") => handle_exit_command(&command), //guard matching any "exit x" command where x stands for status code
            x  if x.to_string().contains("echo") => handle_echo_command(&command), //guard matching any "echo x" command where x stands string to be printed
            x  if x.to_string().contains("type") => handle_type_command(&command), //guard matching any "type x" command where x stands for string to be printed with specific type
            _ => println!("{command}: command not found"), //default case where command is not implemented
        }
    } else {
        //case when someone inserts only one command which can be exit or nothing (for now)
        match input {
            x  if x.to_string().contains("exit") => handle_exit_command(&input), //guard matching any "exit x" command where x stands for status code
            _ => println!("{input}: command not found"), //default case where command is not implemented
        }
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
