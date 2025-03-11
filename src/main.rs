#[allow(unused_imports)]
use std::io::{self, Write};

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
            // echo builtin
            let mut parts = command.split_whitespace();
            parts.next(); // Skip "echo"
            let output: Vec<&str> = parts.collect();
            println!("{}", output.join(" "));
            continue; // Skip further processing
        } else if command == "exit 0" {
            std::process::exit(0);
        } else if command.starts_with("type") {
            // type builtin
            let mut parts = command.split_whitespace();
            parts.next(); // Skip "type"
            if let Some(cmd) = parts.next() {
                match cmd {
                    "echo" | "exit" | "type" | "pwd" => {
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
        }
        
          else {
            // For any unrecognized command
            // println!("{}: command not found", command);
            let mut parts = command.split_whitespace();
            if let Some(prog) = parts.next(){
                let args: Vec<&str> = parts.collect();
                match std::process::Command::new(prog).args(&args).status() {
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
