use std::{
    io::Write,
    process::{Command, Output, Stdio},
};

use serde_json::json;

const INPUTS: [(&str, &str); 3] = [
    ("json", r#"{"name":"transformer","enabled":true}"#),
    ("toml", "name = \"transformer\"\nenabled = true\n"),
    ("yaml", "name: transformer\nenabled: true\n"),
];

fn run(args: &[&str], input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_transformer"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
    child.wait_with_output().unwrap()
}

fn stdout(output: Output, from: &str, to: &str) -> String {
    assert!(
        output.status.success(),
        "{from} -> {to} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn expected() -> serde_json::Value {
    json!({"name": "transformer", "enabled": true})
}

#[test]
fn every_input_converts_to_json() {
    for (from, input) in INPUTS {
        let output = stdout(run(&["--from", from, "json"], input), from, "json");
        assert_eq!(serde_json::from_str::<serde_json::Value>(&output).unwrap(), expected());
    }
}

#[test]
fn every_input_converts_to_toml() {
    for (from, input) in INPUTS {
        let output = stdout(run(&["--from", from, "toml"], input), from, "toml");
        let value: toml::Value = toml::from_str(&output).unwrap();
        assert_eq!(serde_json::to_value(value).unwrap(), expected());
    }
}

#[test]
fn every_input_converts_to_yaml() {
    for (from, input) in INPUTS {
        let output = stdout(run(&["--from", from, "yaml"], input), from, "yaml");
        let value: serde_yml::Value = serde_yml::from_str(&output).unwrap();
        assert_eq!(serde_json::to_value(value).unwrap(), expected());
    }
}

#[test]
fn every_input_generates_go_types() {
    for (from, input) in INPUTS {
        let output = stdout(run(&["--from", from, "go", "--type", "Config"], input), from, "go");
        assert!(output.starts_with("type Config struct"));
        assert!(output.contains("Name string `json:\"name\"`"));
        assert!(output.contains("Enabled bool `json:\"enabled\"`"));
    }
}

#[test]
fn every_input_generates_rust_serde_structs() {
    for (from, input) in INPUTS {
        let output =
            stdout(run(&["--from", from, "serde", "--name", "Config"], input), from, "serde");
        assert!(output.contains("#[derive(Debug, Serialize, Deserialize)]"));
        assert!(output.contains("pub struct Config"));
        assert!(output.contains("pub name: String"));
        assert!(output.contains("pub enabled: bool"));
    }
}

#[test]
fn input_format_is_guessed_for_every_supported_format() {
    for (from, input) in INPUTS {
        let output = stdout(run(&[], input), from, "guessed json");
        assert_eq!(serde_json::from_str::<serde_json::Value>(&output).unwrap(), expected());
    }
}
