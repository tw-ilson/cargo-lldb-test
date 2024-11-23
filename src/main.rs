use std::process::Command;
use std::env;
use serde_json;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: cargo lldb-test <test_name> [additional_options]");
        return;
    }

    let test_name = &args[2];
    let additional_options: Vec<&String> = args[3..].iter().collect();

    // Step 1: Build the tests without running them
    let mut build_command = Command::new("cargo");
    build_command.args(&["test", test_name, "--no-run"]);
    build_command.args(&additional_options);
    
    let build_output = build_command.output().expect("Failed to execute cargo test --no-run");
    if !build_output.status.success() {
        eprintln!("Failed to build tests");
        return;
    }

    // Step 2: Find the test binary
    let mut find_command = Command::new("cargo");
    find_command.args(&["test", "--no-run", "--message-format=json"]);
    find_command.args(&additional_options);
    
    let find_output = find_command.output().expect("Failed to find test binary");
    let test_binary = String::from_utf8_lossy(&find_output.stdout)
        .lines()
        .find_map(|line| {
            serde_json::from_str::<serde_json::Value>(line).ok()
                .and_then(|v| v["executable"].as_str().map(String::from))
        })
        .expect("Failed to find test binary path");

    // Step 3: Run the specific test using LLDB
    let mut lldb_command = Command::new("rust-lldb");
    lldb_command.arg(&test_binary);

    let status = lldb_command.status().expect("Failed to execute rust-lldb");

    std::process::exit(status.code().unwrap_or(1));
}

#[test]
fn dummy() {
    assert!(true)
}
