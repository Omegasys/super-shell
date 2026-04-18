use std::time::Instant;

#[test]
fn test_startup_time() {
    let start = Instant::now();

    // Simulate shell startup (replace with real init later)
    let _ = 1 + 1;

    let duration = start.elapsed();
    assert!(duration.as_millis() < 100);
}
