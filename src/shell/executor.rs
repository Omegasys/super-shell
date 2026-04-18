use std::process::Command as ProcessCommand;
use crate::shell::parser::Command;
use crate::shell::builtins;

pub fn execute(cmd: Command) {
    // Check builtins first
    if builtins::handle_builtin(&cmd) {
        return;
    }

    let mut child = match ProcessCommand::new(&cmd.program)
        .args(&cmd.args)
        .spawn()
    {
        Ok(child) => child,
        Err(e) => {
            eprintln!("Error executing command: {}", e);
            return;
        }
    };

    if let Err(e) = child.wait() {
        eprintln!("Failed to wait on child process: {}", e);
    }
}
