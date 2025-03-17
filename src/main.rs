#[allow(unused_imports)]
use std::io::{self, Write};

pub fn parse_command(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for c in input.chars() {
        match c {
            '\'' => {
                in_quotes = !in_quotes; // Toggle quoted state
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                current.push(c);
            }
        }
    }
    // Push the last token if there is one
    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}


fn main() {
    // Wait for user input
    let stdin = io::stdin();

    loop {
        // Print prompt inside the loop so it's shown on every iteration
        print!("$ ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        if stdin.read_line(&mut input).unwrap() == 0 {
            break;
        }
        let command = input.trim();

        if command.starts_with("echo") {
            // Instead of using split_whitespace, call your parser
            let tokens = parse_command(command);
            // The first token should be "echo", so skip it
            let output = tokens.into_iter().skip(1).collect::<Vec<_>>();
            println!("{}", output.join(" "));
            continue;
            // echo builtin
            // let mut parts = command.split_whitespace();
            // parts.next(); // Skip "echo"
            // let output: Vec<&str> = parts.collect();
            // println!("{}", output.join(" "));
            // continue; // Skip further processing
        } else if command == "exit 0" {
            std::process::exit(0);
        } else if command.starts_with("type") {
            // type builtin
            let mut parts = command.split_whitespace();
            parts.next(); // Skip "type"
            if let Some(cmd) = parts.next() {
                match cmd {
                    "echo" | "exit" | "type" | "pwd" | "cd" => {
                        println!("{} is a shell builtin", cmd);
                    },
                    _ => {
                        let path_env = std::env::var("PATH").unwrap_or_default();
                        let mut found = false;
                        for dir in path_env.split(':') {
                            let candidate = std::path::Path::new(dir).join(cmd);
                            if candidate.exists() && candidate.is_file() {
                                println!("{} is {}", cmd, candidate.display());
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            println!("{}: not found", cmd);
                        }
                    },
                }
            } else {
                println!("type: not enough arguments");
            }
            continue; // Skip further processing
        } else if command.starts_with("pwd") {
            match std::env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                 Err(e) => println!("Error getting current directory: {}", e),
            }
            continue; //x
        } else if command.starts_with("cd") {
            let mut parts = command.split_whitespace();
            parts.next(); // skip cd
            let output: Vec<&str> = parts.collect(); // if cmd cd /usr/local/bin then output will be vector containing /usr/local/bin
            if output.is_empty() {
                println!("cd: No directory provided");
            } else if !output.is_empty() && output[0].starts_with("/"){
                // TODO: proceed to change the directory
                match std::env::set_current_dir(output[0]) {
                    Ok(()) => { /* directory changed successfully, do nothing */ },
                    Err(_) => {
                        println!("cd: {}: No such file or directory", output[0]);
                    },
                }
                
            } else if output[0] == "~" {
                let home_dir = std::env::var("HOME");
                match std::env::var("HOME") {
                    Ok(home_dir) => {
                        match std::env::set_current_dir(&home_dir) {
                            Ok(()) => {/*success: directory changed to home_dir*/},
                            Err(_) => {
                                println!("cd: ~: No such file or directory");
                            },
                        }
                    },
                    Err(_) => {
                        println!("cd: ~: No such file or directory");
                    }
                }
            }
              else { // cases when user types cd foo or just cd
                // TODO: optionally handle the error cases or print error message
                // relative path branch
                if !output.is_empty() {
                    // change to the relative directory
                    match std::env::set_current_dir(output[0]) {
                        Ok(()) => {/* success: directory changed */},
                        Err(_) => {
                            println!("cd: {}: No such file or directory", output[0]);
                        }
                    }
                } else{
                    println!("cd: No directory provided");
                }
                
            }
         }
        
          else {
            // For any unrecognized command
            // println!("{}: command not found", command);
            //let mut parts = command.split_whitespace();
            let tokens = parse_command(command);
            if let Some(prog) = tokens.get(0) {
                let args = &tokens[1..];
                match std::process::Command::new(prog).args(args).status() {
                    Ok(status) => {
                        // optionally check status or do nothing
                        if !status.success() {
                            // command didn't exit successfully
                            // optionally check the actual exit code
                            if let Some(code) = status.code() {
                                println!("Command {} exited with code {}", prog, code);
                            } else {
                                println!("Command {} terminated by signal", prog);
                            }
                        }
                    },
                    Err(_) => {
                        println!("{}: command not found", prog);
                    }
                }
            }
        }
        input.clear();
    }
}
