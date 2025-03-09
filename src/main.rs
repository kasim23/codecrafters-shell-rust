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

        // For now every command is considered invalid
        println!("{}: command not found", command);
        input.clear();
    }

}
