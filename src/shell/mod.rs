pub mod parser;
pub mod executor;
pub mod builtins;

use std::io::{self, Write};
use parser::parse_command;
use executor::execute;

pub fn run_shell() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input");
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        if input == "exit" {
            println!("Exiting shell...");
            break;
        }

        let command = parse_command(input);
        execute(command);
    }
}
