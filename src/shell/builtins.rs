use std::env;
use crate::shell::parser::Command;

pub fn handle_builtin(cmd: &Command) -> bool {
    match cmd.program.as_str() {
        "cd" => {
            let target = cmd.args.get(0).map(|s| s.as_str()).unwrap_or("/");
            if let Err(e) = env::set_current_dir(target) {
                eprintln!("cd error: {}", e);
            }
            true
        }

        "pwd" => {
            match env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(e) => eprintln!("pwd error: {}", e),
            }
            true
        }

        "echo" => {
            println!("{}", cmd.args.join(" "));
            true
        }

        "help" => {
            println!("Built-in commands:");
            println!("  cd <dir>   - change directory");
            println!("  pwd        - print current directory");
            println!("  echo <msg> - print message");
            println!("  exit       - exit shell");
            true
        }

        _ => false,
    }
}
