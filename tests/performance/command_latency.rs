use std::time::Instant;
use std::process::Command;

#[test]
fn test_command_latency() {
    let start = Instant::now();

    let _ = Command::new("echo")
        .arg("test")
        .output()
        .expect("failed");

    let duration = start.elapsed();
    assert!(duration.as_secs() < 1);
}
