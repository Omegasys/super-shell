use std::process::Command;

#[test]
fn stress_test_many_commands() {
    for _ in 0..100 {
        let output = Command::new("echo")
            .arg("stress")
            .output()
            .expect("failed");

        assert!(output.status.success());
    }
}
