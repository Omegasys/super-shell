use std::process::Command;

#[test]
fn test_ls_command_runs() {
    let output = Command::new("ls")
        .output()
        .expect("failed to run ls");

    assert!(output.status.success());
}
