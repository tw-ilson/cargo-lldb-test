use std::process::Command;
use std::env;
use std::io::{stdout, stderr, Write};
use serde_json;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: cargo lldb-test <test_name> [additional_options]");
        return;
    }

    let test_name = &args[2];
    println!("LLDB Test: {}", test_name);
    let additional_options: Vec<&String> = args[3..].iter().collect();

    // Step 1: Build the tests without running them
    let mut build_command = Command::new("cargo");
    build_command.args(&["test", test_name]);
    build_command.args(&additional_options);
    build_command.args(&["--no-run", "--message-format=json"]);
    
    let build_output = build_command.output().expect("Failed to execute cargo test --no-run");
    stdout().write_all(&build_output.stdout).unwrap();
    stderr().write_all(&build_output.stderr).unwrap();
    if !build_output.status.success() {
        eprintln!("Failed to build tests");
        return;
    }

    let test_binary = String::from_utf8_lossy(&build_output.stdout)
        .lines()
        .find_map(|line| {
            serde_json::from_str::<serde_json::Value>(line).ok()
                .and_then(|v| v["executable"].as_str().map(String::from))
        })
        .expect("Failed to find test binary path");

    // Step 3: Run the specific test using LLDB
    let mut lldb_command = Command::new("rust-lldb");
    lldb_command.args(&["--", &test_binary, test_name]);

    let status = lldb_command.status().expect("Failed to execute rust-lldb");

    std::process::exit(status.code().unwrap_or(1));
}

#[test]
fn dummy() {
    assert!(true)
}
