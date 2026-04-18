use super_shell_project::shell::parser::parse_command;

#[test]
fn test_parse_simple_command() {
    let cmd = parse_command("echo hello");
    assert_eq!(cmd.program, "echo");
    assert_eq!(cmd.args, vec!["hello"]);
}

#[test]
fn test_parse_multiple_args() {
    let cmd = parse_command("ls -la /tmp");
    assert_eq!(cmd.program, "ls");
    assert_eq!(cmd.args, vec!["-la", "/tmp"]);
}

#[test]
fn test_empty_input() {
    let cmd = parse_command("");
    assert_eq!(cmd.program, "");
    assert!(cmd.args.is_empty());
}
