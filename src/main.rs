#[allow(unused_imports)]
use std::io::{self, Write};
use std::fs::File;
use std::fs::OpenOptions;


// this gives you three distinct states, None: not inside any quotes, Single: inside single quotes, Double: inside double quotes
#[derive(Debug, PartialEq)]
enum QuoteState {
    None,
    Single,
    Double,
}

pub fn parse_command(input: &str) -> Vec<String> {
    /*Instead of matching only on ' ' and '\'', you’ll now match on:
    - A single quote (') and check if you’re in None or Double state (in Double, it’s literal).
    - A double quote (") and check if you’re in None or Single state (in Single, it’s literal).
    - A backslash (\) when in Double state.
    - The space character, but only if you're in None. */
    
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut state = QuoteState::None;
    // Create a peekable iterator over the input characters.
    let mut iter = input.chars().peekable();

    while let Some(c) = iter.next() {
        match c {
            '\'' => {
                // Handle single quotes.
                match state {
                    QuoteState::None => state = QuoteState::Single,
                    QuoteState::Single => state = QuoteState::None,
                    QuoteState::Double => current.push(c), // In double quotes, single quotes are literal.
                }
            },
            '"' => {
                // Handle double quotes.
                match state {
                    QuoteState::None => state = QuoteState::Double,
                    QuoteState::Double => state = QuoteState::None,
                    QuoteState::Single => current.push(c), // In single quotes, double quotes are literal.
                }
            },
            '\\' => {
                // Handle backslash escapes.
                match state {
                    QuoteState::Double => {
                        // In double quotes, check if the next character should be escaped.
                        if let Some(&next_char) = iter.peek() {
                            match next_char {
                                '\\' | '$' | '"' | '\n' => {
                                    iter.next(); // Consume the next character.
                                    current.push(next_char);
                                },
                                _ => {
                                    // Not escapable, so treat the backslash as literal.
                                    current.push('\\');
                                }
                            }
                        } else {
                            current.push('\\');
                        }
                    },
                    QuoteState::None => {
                        // Outside any quotes, backslash escapes the next character.
                        if let Some(&next_char) = iter.peek() {
                            iter.next(); // Consume the escaped character.
                            current.push(next_char);
                        } else {
                            current.push('\\');
                        }
                    },
                    QuoteState::Single => {
                        // Inside single quotes, backslash is literal.
                        current.push('\\');
                    }
                }
            },
            ' ' => {
                // Space is a delimiter only when not inside quotes.
                if state == QuoteState::None {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                } else {
                    current.push(c);
                }
            },
            _ => {
                // All other characters are appended to the current token.
                current.push(c);
            }
        }
    }
    // If there's a leftover token, push it into tokens.
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
            let tokens = parse_command(command);
            if let Some(pos) = tokens.iter().position(|x| x == ">" || x == "1>") {
                let command_part = tokens[..pos].to_vec();
                let output_file = tokens.get(pos + 1).cloned();
                // Process redirection: open file, execute command with stdout redirected.
                if let Some(file_token) = output_file {
                    let file = File::create(&file_token).unwrap_or_else(|e| {
                        println!("Error opening {}: {}", file_token, e);
                        std::process::exit(1);
                    });
                    // Extract the echo arguments from the command part (skip the "echo" token).
                    let output = command_part.into_iter().skip(1).collect::<Vec<_>>();
                    // Instead of printing to stdout, write to the file.
                    use std::io::Write;
                    writeln!(&mut &file, "{}", output.join(" ")).unwrap();
                    continue; // Skip normal echo processing.
                } else {
                    println!("Error: no output file specified after redirection operator");
                    continue;
                }
            } else if let Some(pos) = tokens.iter().position(|x| x == "2>") {
                let command_part = tokens[..pos].to_vec();
                let output_file = tokens.get(pos + 1).cloned();
                // process redirection stderr
                if let  Some(file_token) = output_file {
                    // open file for writing
                    let file = File::create(&file_token).unwrap_or_else(|e| {
                        println!("Error openeing {}: {}", file_token, e);
                        std::process::exit(1);
                    });
                    let executable = command_part[0].clone();
                    let args = &command_part[1..];
                    // redirect stderr to file
                    std::process::Command::new(executable)
                        .args(args)
                        .stderr(file)
                        .status();
                    continue; // skip normal execution
                } else {
                    println!("Error: no output file specified after redirection operator");
                    continue;
                }
            } else if let Some(pos) = tokens.iter().position(|x| x == ">>" || x == "1>>") {
                let command_part = tokens[..pos].to_vec();
                let output_file = tokens.get(pos + 1).cloned();
                // process appending stdout
                if let Some(file_token) = output_file {
                    //  use Rust's std::fs::OpenOptions to open the file with the append flag set
                    let file = OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(&file_token)
                        .unwrap_or_else(|e| {
                            println!("Error openeing {}: {}", file_token, e);
                            std::process::exit(1);
                        });
                    // extract the executable and arguments from the command part
                    let executable = command_part[0].clone();
                    let args = &command_part[1..];
                    std::process::Command::new(executable)
                        .args(args)
                        .stdout(file)
                        .status();
                    continue; //skip normal execution
                } else {
                    println!("Error: no output file specified after redirection operator");
                    continue;
                }
            }
            
              else {
                // Process as a normal echo command.
                let output = tokens.into_iter().skip(1).collect::<Vec<_>>();
                println!("{}", output.join(" "));
                continue;
            }
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
                let _home_dir = std::env::var("HOME");
                match std::env::var("HOME") {
                    Ok(_home_dir) => {
                        match std::env::set_current_dir(&_home_dir) {
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
            
            if let Some(pos) = tokens.iter().position(|x| x == ">" || x == "1>") {
                let command_part = tokens[..pos].to_vec();
                let output_file = tokens.get(pos + 1).cloned();
                // Process redirection: open file, execute command with stdout redirected.
                if let Some(file_token) = output_file {
                    // Open the file for writing (using std::fs::File::create)
                    // Open the file for writing:
                    let file = File::create(&file_token).unwrap_or_else(|e| {
                        println!("Error opening {}: {}", file_token, e);
                        std::process::exit(1);
                    });
                    // Now extract the executable and arguments:
                    let executable = command_part[0].clone();
                    let args = &command_part[1..];
                    // And then use .stdout(file_handle) on your Command. Execute the command with redirection
                    std::process::Command::new(executable)
                        .args(args)
                        .stdout(file)
                        .status();

                    // skip the normal command execution
                    continue;
                } else {
                    println!("Error: no output file specified after redirection operator");
                    continue; // Skip further processing.
                }
            } else if let Some(pos) = tokens.iter().position(|x| x == "2>") {
                let command_part = tokens[..pos].to_vec();
                let output_file = tokens.get(pos + 1).cloned();
                // process redirection stderr
                if let  Some(file_token) = output_file {
                    // open file for writing
                    let file = File::create(&file_token).unwrap_or_else(|e| {
                        println!("Error openeing {}: {}", file_token, e);
                        std::process::exit(1);
                    });
                    let executable = command_part[0].clone();
                    let args = &command_part[1..];
                    // redirect stderr to file
                    std::process::Command::new(executable)
                        .args(args)
                        .stderr(file)
                        .status();
                    continue; // skip normal execution
                } else {
                    println!("Error: no output file specified after redirection operator");
                    continue;
                }
            } else if let Some(pos) = tokens.iter().position(|x| x == ">>" || x == "1>>") {
                let command_part = tokens[..pos].to_vec();
                let output_file = tokens.get(pos + 1).cloned();
                // process appending stdout
                if let Some(file_token) = output_file {
                    //  use Rust's std::fs::OpenOptions to open the file with the append flag set
                    let file = OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(&file_token)
                        .unwrap_or_else(|e| {
                            println!("Error openeing {}: {}", file_token, e);
                            std::process::exit(1);
                        });
                    // extract the executable and arguments from the command part
                    let executable = command_part[0].clone();
                    let args = &command_part[1..];
                    std::process::Command::new(executable)
                        .args(args)
                        .stdout(file)
                        .status();
                    continue; //skip normal execution
                } else {
                    println!("Error: no output file specified after redirection operator");
                    continue;
                }
            }
            
            else {
                // Process as a normal command.
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

        }
        input.clear();
    }
}
