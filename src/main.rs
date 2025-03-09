#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {

    // Wait for user input
    let stdin = io::stdin();
    

    loop{
        // Print prompt inside the loop so it's shown on every iteration
        print!("$ ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();

        if stdin.read_line(&mut input).unwrap() == 0{
            break;
        }

        let command = input.trim();


        // echo builtin added
        if command.starts_with("echo"){
            // split command into words
            let mut parts = command.split_whitespace();
            // if first part is echo, skip it
            parts.next();
            // the rest is what we want to echo
            let output: Vec<&str> = parts.collect();
            println!("{}", output.join(" "));
            continue; // Skip printing the error message below
        } else if command == "exit 0" {
            std::process::exit(0); // or break;
        }

        // For now every command is considered invalid
        println!("{}: command not found", command);
        input.clear();
    }

}
