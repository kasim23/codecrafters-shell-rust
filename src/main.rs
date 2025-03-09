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

        // echo builtin added
        if command.starts_with("echo") {
            // Split command into words.
            let mut parts = command.split_whitespace();
            parts.next(); // Skip the "echo" part.
            // The rest is what we want to echo.
            let output: Vec<&str> = parts.collect();
            println!("{}", output.join(" "));
            continue; // Skip printing the error message below.
        } else if command == "exit 0" {
            std::process::exit(0); // Exits with code 0.
        } else if command.starts_with("type") {
            let mut parts = command.split_whitespace();
            parts.next(); // Skip the "type" command itself.
            if let Some(cmd) = parts.next() { // Check if there's an argument.
                match cmd {
                    "echo" | "exit" | "type" => println!("{} is a shell builtin", cmd),
                    _ => println!("{}: not found", cmd),
                }
            } else {
                // Print error if no argument is provided.
                println!("type: not enough arguments");
            }
            continue; // Skip printing the error message below.
        }

        // For now, every command that isn't built in is considered invalid.
        println!("{}: command not found", command);
        input.clear();
    }
}
