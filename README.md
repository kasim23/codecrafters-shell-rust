# Codecrafters Shell in Rust

Welcome to the Codecrafters Shell, a Rust-based command line shell project designed to help you learn Rust, deepen your understanding of systems programming, and build a fully functional shell step by step.

## Note

This is a learning project, aimed at learning Rust through project-based learning. It is on-going and I will be adding more functionalities and features to it as the project progresses.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Project Structure](#project-structure)
- [Workflow & Implementation](#workflow--implementation)
- [Installation & Running](#installation--running)
- [Future Enhancements](#future-enhancements)
- [Learning Resources](#learning-resources)
- [License](#license)

## Overview

The Codecrafters Shell is a hands-on project aimed at improving your Rust programming skills. This project involves building a custom shell from scratch. Throughout the project, you'll learn about:

- Input parsing and command handling
- Builtin commands such as `echo`, `cd`, `pwd`, `type`, and `exit`
- Handling external command execution using Rust’s `std::process::Command`
- Working with environment variables and error handling

## Features

- **Builtin Commands:**  
  - **echo:** Prints the provided arguments.  
  - **cd:** Changes the current working directory.  
    - Supports absolute paths (e.g., `/usr/local/bin`).
    - Supports relative paths (e.g., `./`, `../`, `dir`).
    - Supports home directory resolution using the tilde (`~`), reading from the `HOME` environment variable.
  - **pwd:** Displays the current working directory.
  - **type:** Identifies whether a command is a shell builtin or an external executable by searching the PATH.
  - **exit:** Exits the shell with a status code.

- **External Command Execution:**  
  If a command is not recognized as a builtin, the shell attempts to execute it as an external program, leveraging the PATH environment variable for lookup.

## Project Structure

```
codecrafters-shell-rust/
├── src
│   └── main.rs        # Main source file containing the shell implementation
├── Cargo.toml         # Cargo configuration file
└── README.md          # Project documentation
```

## Workflow & Implementation

1. **Input Loop:**  
   The shell continuously reads user input, displays a prompt, and processes commands.

2. **Command Parsing:**  
   Input is trimmed and split into tokens using Rust's standard string methods. This allows the shell to identify the command and its arguments.

3. **Builtin Command Handling:**  
   Each builtin command (`echo`, `exit`, `type`, `pwd`, and `cd`) is handled in its own branch:
   - **echo:** Joins and prints arguments.
   - **exit:** Terminates the shell.
   - **type:** Checks if a command is a builtin or resolves the command's path from the PATH variable.
   - **pwd:** Uses `std::env::current_dir()` to print the current directory.
   - **cd:**  
     - **Absolute paths:** Checked by verifying if the argument starts with `/`.
     - **Relative paths:** Processed directly using `std::env::set_current_dir()`.
     - **Home directory (`~`):** The shell retrieves the `HOME` environment variable and attempts to change the directory accordingly.
  
4. **External Command Execution:**  
   If the command is not a recognized builtin, the shell uses `std::process::Command` to run the command. Errors are caught and a generic error message is printed if the command is not found.

## Installation & Running

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/yourusername/codecrafters-shell-rust.git
   cd codecrafters-shell-rust
   ```

2. **Build the Project:**

   ```bash
   cargo build --release
   ```

3. **Run the Shell:**

   ```bash
   ./target/release/codecrafters-shell-rust
   ```

4. **Usage Example:**

   ```bash
   $ echo Hello, World!
   Hello, World!
   $ cd /usr/local/bin
   $ pwd
   /usr/local/bin
   $ cd ~
   $ pwd
   /home/yourusername
   $ type echo
   echo is a shell builtin
   $ invalid_command
   invalid_command: command not found
   $ exit 0
   ```

## Future Enhancements

- **Relative Paths Enhancements:**  
  Expand handling of more complex relative paths.
- **Tilde Expansion:**  
  Support for tilde expansion in combination with additional path components (e.g., `~/Documents`).
- **Job Control & Piping:**  
  Implement features like background processes and command piping.
- **Improved Error Handling:**  
  More detailed and user-friendly error messages.

## Learning Resources

- [The Rust Programming Language](https://doc.rust-lang.org/book/) – The official Rust book.
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) – Hands-on examples of Rust code.
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/) – Comprehensive reference for Rust’s APIs.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
