use std::process::Command;

#[test]
fn test_cli_dispatch() {
    // CARGO_BIN_EXE is populated by cargo
    let output = Command::new(env!("CARGO_BIN_EXE_cli"))
        .args(&["test", "hello"])
        .output()
        .expect("Failed to run CLI");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("hello"));
}

