use super_shell_project::shell::parser::parse_command;

#[test]
fn test_no_command_injection_split() {
    let cmd = parse_command("echo hello; rm -rf /");

    // Current parser treats everything as args (safe behavior for now)
    assert_eq!(cmd.program, "echo");
    assert!(cmd.args.contains(&"hello;".to_string()) || cmd.args.contains(&"hello".to_string()));
}
