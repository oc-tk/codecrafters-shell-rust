use std::env;
use std::process;
use std::io::{self, Write};

enum Command {
    Echo,
    Type,
    Exit,
}

impl Command {
    // Convert a &str to a Command enum
    fn from_str(command: &str) -> Option<Command> {
        match command {
            "echo" => Some(Command::Echo),
            "type" => Some(Command::Type),
            "exit" => Some(Command::Exit),
            _ => None,
        }
    }

    // Convert the Command enum to a &str
    // fn as_str(&self) -> &str {
    //     match self {
    //         Command::Echo => "echo",
    //         Command::Type => "type",
    //         Command::Exit => "exit",
    //     }
    // }
}

fn handle_exit_command(command: &str) {
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
    let exec_command = command.replace("echo ", "");
    println!("{exec_command}");
}

fn handle_type_command(command: &str) {
    let exec_command = command.replace("type ", "");
    if let Some(_) = Command::from_str(&exec_command) {
        println!("{exec_command} is a shell builtin");
    } else {
        if !check_path(&exec_command) {
            println!("{exec_command} not found")
        }
    }
}

fn check_path(executable: &str) -> bool {

    if let Ok(paths) = env::var("PATH")
    {
        for path in paths.split(':') {
            let path = format!("{}/{}", path, executable);
            if std::path::Path::new(&path).exists() {
                println!("{} is in {}", executable, path);
                return true;
            }
        }
    }
    false

    // match (env::var("PATH"), executable) {
    //     (Ok(paths), cmd) => {
    //         for path in paths.split(':') {
    //             let path = format!("{}/{}", path, cmd);
    //             if std::path::Path::new(&path).exists() {
    //                 println!("{} is in {}", cmd, path);
    //                 return true;
    //             }
    //         }
    //         false
    //     }
    //     _ => false,
    // }
}

//handle pattern matching
fn handle_matching(input: &str) {
    if let Some(command) = Command::from_str(input.split_whitespace().next().unwrap()) {
        match command {
            Command::Echo => handle_echo_command(&input),
            Command::Type => handle_type_command(&input),
            Command::Exit => handle_exit_command(&input),
        }
    } else {
        println!("{input}: command not found")
    }

    // if let Some(first_word) = input.split_whitespace().next() {
        

    //     //normally interpreting first part of entry
    //     match command {
    //         x  if x.to_string().contains("exit") => handle_exit_command(&input), //guard matching any "exit x" command where x stands for status code
    //         x  if x.to_string().contains("echo") => handle_echo_command(&input), //guard matching any "echo x" command where x stands string to be printed
    //         x  if x.to_string().contains("type") => handle_type_command(&input), //guard matching any "type x" command where x stands for string to be printed with specific type
    //         _ => println!("{input}: command not found"), //default case where command is not implemented
    //     }
    // } else {
    //     //case when someone inserts only one command which can be exit or nothing (for now)
    //     match input {
    //         x  if x.to_string().contains("exit") => handle_exit_command(&input), //guard matching any "exit x" command where x stands for status code
    //         _ => println!("{input}: command not found"), //default case where command is not implemented
    //     }
    // }
    
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
