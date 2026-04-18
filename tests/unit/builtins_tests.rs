use super_shell_project::shell::parser::Command;
use super_shell_project::shell::builtins::handle_builtin;

#[test]
fn test_echo_builtin() {
    let cmd = Command {
        program: "echo".into(),
        args: vec!["hello".into()],
    };

    assert!(handle_builtin(&cmd));
}

#[test]
fn test_pwd_builtin() {
    let cmd = Command {
        program: "pwd".into(),
        args: vec![],
    };

    assert!(handle_builtin(&cmd));
}

#[test]
fn test_unknown_builtin() {
    let cmd = Command {
        program: "not_a_builtin".into(),
        args: vec![],
    };

    assert!(!handle_builtin(&cmd));
}
