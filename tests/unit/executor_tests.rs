use super_shell_project::shell::parser::Command;
use super_shell_project::shell::executor::execute;

#[test]
fn test_execute_echo() {
    let cmd = Command {
        program: "echo".into(),
        args: vec!["hello".into()],
    };

    // Should not panic
    execute(cmd);
}

#[test]
fn test_execute_invalid_command() {
    let cmd = Command {
        program: "this_command_does_not_exist".into(),
        args: vec![],
    };

    // Should fail gracefully, not panic
    execute(cmd);
}
