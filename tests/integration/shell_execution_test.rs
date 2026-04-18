use std::process::Command;

#[test]
fn test_echo_command_integration() {
    let output = Command::new("echo")
        .arg("hello")
        .output()
        .expect("failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "hello");
}
