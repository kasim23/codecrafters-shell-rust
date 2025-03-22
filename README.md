# Codecrafters Shell in Rust

Welcome to the Codecrafters Shell, a Rust-based command line shell project designed to help you learn Rust, deepen your understanding of systems programming, and build a fully functional shell step by step.

## Note

This is a learning project aimed at mastering Rust through project-based learning. The shell is under active development, and more functionalities will be added as the project progresses.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Quoting and Escapes](#quoting-and-escapes)
- [Output Redirection](#output-redirection)
- [Project Structure](#project-structure)
- [Workflow & Implementation](#workflow--implementation)
- [Installation & Running](#installation--running)
- [Usage Examples](#usage-examples)
- [Future Enhancements](#future-enhancements)
- [Learning Resources](#learning-resources)
- [License](#license)

## Overview

The Codecrafters Shell is a hands-on project aimed at improving your Rust programming skills. This project involves building a custom shell from scratch. Throughout the project, you'll learn about:

- Input parsing and command handling
- Builtin commands such as `echo`, `cd`, `pwd`, `type`, and `exit`
- Handling external command execution using Rust’s `std::process::Command`
- Advanced tokenization that supports quoting and escaping
- Output redirection and file I/O
- Working with environment variables and error handling

## Features

- **Builtin Commands:**  
  - **echo:** Prints the provided arguments.
  - **cd:** Changes the current working directory.
    - Supports absolute paths (e.g., `/usr/local/bin`).
    - Supports relative paths (e.g., `./`, `../`, `dir`).
    - Supports home directory resolution using the tilde (`~`), by reading the `HOME` environment variable.
  - **pwd:** Displays the current working directory.
  - **type:** Identifies whether a command is a shell builtin or an external executable by searching the PATH.
  - **exit:** Exits the shell with a status code.
  
- **External Command Execution:**  
  If a command is not recognized as a builtin, the shell attempts to execute it as an external program by leveraging the PATH environment variable.

- **Quoting and Escapes:**  
  - **Single Quotes:**  
    Preserve the literal value of all characters within the quotes, including backslashes.
    
    **Example:**  
    ```
    $ echo 'shell\nscript'
    shell\nscript
    ```
  
  - **Double Quotes:**  
    Preserve the literal value of all characters except that backslashes can escape characters like `\`, `$`, `"`, or newline.
    
    **Example:**  
    ```
    $ echo "quz  hello" "bar"
    quz  hello bar
    $ echo "bar" "shell's" "foo"
    bar shell's foo
    ```

  - **Backslashes Outside Quotes:**  
    Outside of any quoting, a backslash escapes the next character.
    
    **Example:**  
    ```
    $ echo world\ \ \ \ script
    world     script
    ```

- **Executing Quoted Executables:**  
  The shell supports executing executables whose names are quoted (and may contain spaces, quotes, or backslashes).

  **Example:**  
  ```
  $ 'exe with "quotes"' file
  (executes the renamed executable and prints its output)
  ```

- **Output Redirection:**  
  The shell supports redirecting the standard output of a command to a file using `>` or `1>`.
  
  **Examples:**  
  ```
  $ ls /tmp/baz > /tmp/foo/baz.md
  $ cat /tmp/foo/baz.md
  apple
  blueberry
  $ echo 'Hello James' 1> /tmp/foo/foo.md
  $ cat /tmp/foo/foo.md
  Hello James
  ```

## Quoting and Escapes

The shell implements a custom parser to handle tokenization that respects:
- Single quotes: Everything inside is literal.
- Double quotes: Most characters are literal except that backslashes escape specific characters.
- Backslashes: Outside quotes, a backslash escapes the next character; inside double quotes, specific characters are escaped; inside single quotes, backslashes are treated literally.

## Output Redirection

When a command contains the `>` or `1>` operator, the shell splits the command into two parts:
- **Command Part:** The executable and its arguments.
- **Redirection Target:** The file path immediately following the redirection operator.

For example, in:
```
ls /tmp/baz > /tmp/foo/baz.md
```
- The command part is: `ls /tmp/baz`
- The output is redirected to `/tmp/foo/baz.md` by opening that file and passing its handle as stdout when executing the command.

## Project Structure

```
codecrafters-shell-rust/
├── src
│   └── main.rs        # Main source file containing the shell implementation and custom parser.
├── Cargo.toml         # Cargo configuration file.
└── README.md          # Project documentation.
```

## Workflow & Implementation

1. **Input Loop:**  
   The shell continuously reads user input, displays a prompt, and processes commands.

2. **Command Parsing:**  
   Input is trimmed and passed through a custom parser that splits the input into tokens, respecting quoting and escape rules.

3. **Builtin Command Handling:**  
   - **echo:** Processes the tokens and prints them. Redirection is supported by detecting tokens like `>` or `1>` and redirecting the output to a file.
   - **cd, pwd, type, exit:** Each is handled in its own branch with appropriate use of Rust's standard library.
   
4. **External Command Execution:**  
   Commands not recognized as builtins are executed using `std::process::Command`, and errors are handled gracefully.

5. **Redirection Logic:**  
   After tokenization, the shell scans for redirection operators and, if found, splits the tokens. It then opens the target file using Rust's file I/O and redirects stdout accordingly.

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

## Usage Examples

**Echo with Quoting and Escapes:**

- **Simple Echo:**
  ```bash
  $ echo Hello, World!
  Hello, World!
  ```
- **Echo with Single Quotes (backslashes treated literally):**
  ```bash
  $ echo 'shell\nscript'
  shell\nscript
  ```
- **Echo with Double Quotes (backslashes escape):**
  ```bash
  $ echo "quz  hello" "bar"
  quz  hello bar
  ```
- **Echo with Backslashes Outside Quotes:**
  ```bash
  $ echo world\ \ \ \ script
  world     script
  ```

**Executing Quoted Executables:**
```bash
$ 'exe with "quotes"' file
(content printed by the executable)
```

**Output Redirection:**
- **List Directory with Redirection:**
  ```bash
  $ ls /tmp/baz > /tmp/foo/baz.md
  $ cat /tmp/foo/baz.md
  apple
  blueberry
  ```
- **Echo with Redirection:**
  ```bash
  $ echo 'Hello James' 1> /tmp/foo/foo.md
  $ cat /tmp/foo/foo.md
  Hello James
  ```

## Future Enhancements

- **Advanced Redirection:**  
  Add support for stderr redirection and combined stdout/stderr redirection.
- **Job Control & Piping:**  
  Implement background process handling and command piping.
- **Enhanced Builtin Commands:**  
  Support additional builtins or improve the functionality of existing ones.
- **Relative Path Enhancements:**  
  Further refine the handling of relative paths in the `cd` command.

## Learning Resources

- [The Rust Programming Language](https://doc.rust-lang.org/book/) – The official Rust book.
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) – Hands-on examples of Rust code.
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/) – Comprehensive reference for Rust’s APIs.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
