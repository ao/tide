use assert_cmd::Command;

// Basic integration tests for the CLI application

#[test]
fn test_app_runs() {
    let result = Command::cargo_bin("tide")
        .unwrap()
        .arg("--url")
        .arg("https://example.com")
        .arg("--duration")
        .arg("1") // Short duration for testing
        .assert();

    result.success();
}

#[test]
fn test_app_with_invalid_url() {
    let result = Command::cargo_bin("tide")
        .unwrap()
        .arg("--url")
        .arg("invalid-url")
        .assert();

    result.failure();
}

#[test]
fn test_app_with_retries() {
    // Test with retries parameter
    let result = Command::cargo_bin("tide")
        .unwrap()
        .arg("--url")
        .arg("https://example.com")
        .arg("--duration")
        .arg("1")
        .arg("--retries")
        .arg("5")
        .assert();

    result.success();
}

#[test]
fn test_app_with_timeout() {
    // Test with timeout parameter
    let result = Command::cargo_bin("tide")
        .unwrap()
        .arg("--url")
        .arg("https://example.com")
        .arg("--duration")
        .arg("1")
        .arg("--timeout")
        .arg("2")
        .assert();

    result.success();
}
