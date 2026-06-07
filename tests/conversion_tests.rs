use serde::Serialize;
use serde_json::{from_str as json_from_str, to_value};
use serde_yml::from_str as yaml_from_str;
use toml::from_str as toml_from_str;
use transformer::Format;

const MASTER_TOML: &str = r#"[package]
name = "transformer"
version = "1.2.1"
edition = "2021"
description = "A CLI tool to convert from JSON|TOML|YAML to JSON|TOML|YAML|Rust serde structs."

[dependencies]
anyhow = "1.0"
json_typegen_shared = "0.7.0"
serde_json = "1.0"
serde_yml = "0.0.12"
toml = "0.8"

[dependencies.clap]
version = "4.5"
features = ["derive", "wrap_help"]

[dependencies.serde]
version = "1.0"
features = ["derive"]

[profile.release]
opt-level = "z"
strip = true
lto = true
codegen-units = 1
panic = "abort"
"#;

fn parse_json(s: &str) -> serde_json::Value {
    json_from_str(s).unwrap()
}

fn parse_toml(s: &str) -> toml::Value {
    toml_from_str(s).unwrap()
}

fn parse_yaml(s: &str) -> serde_yml::Value {
    yaml_from_str(s).unwrap()
}

fn to_canonical_json(v: &impl Serialize) -> serde_json::Value {
    to_value(v).unwrap()
}

#[test]
fn toml_to_json() {
    let input = Format::Toml(parse_toml(MASTER_TOML));
    let output = input.to_json();
    let parsed: serde_json::Value = parse_json(&output);
    assert_eq!(to_canonical_json(&parse_toml(MASTER_TOML)), parsed);
}

#[test]
fn toml_to_yaml() {
    let input = Format::Toml(parse_toml(MASTER_TOML));
    let output = input.to_yaml();
    let parsed: serde_yml::Value = parse_yaml(&output);
    assert_eq!(to_canonical_json(&parse_toml(MASTER_TOML)), to_canonical_json(&parsed));
}

#[test]
fn toml_to_toml() {
    let input = Format::Toml(parse_toml(MASTER_TOML));
    let output = input.to_toml();
    let parsed: toml::Value = parse_toml(&output);
    assert_eq!(to_canonical_json(&parse_toml(MASTER_TOML)), to_canonical_json(&parsed));
}

#[test]
fn json_to_toml() {
    let json_input = Format::Toml(parse_toml(MASTER_TOML)).to_json();
    let input = Format::Json(parse_json(&json_input));
    let output = input.to_toml();
    let parsed: toml::Value = parse_toml(&output);
    assert_eq!(to_canonical_json(&parse_toml(MASTER_TOML)), to_canonical_json(&parsed));
}

#[test]
fn json_to_yaml() {
    let json_input = Format::Toml(parse_toml(MASTER_TOML)).to_json();
    let input = Format::Json(parse_json(&json_input));
    let output = input.to_yaml();
    let parsed: serde_yml::Value = parse_yaml(&output);
    assert_eq!(to_canonical_json(&parse_toml(MASTER_TOML)), to_canonical_json(&parsed));
}

#[test]
fn json_to_json() {
    let json_input = Format::Toml(parse_toml(MASTER_TOML)).to_json();
    let input = Format::Json(parse_json(&json_input));
    let output = input.to_json();
    let parsed: serde_json::Value = parse_json(&output);
    assert_eq!(to_canonical_json(&parse_toml(MASTER_TOML)), parsed);
}

#[test]
fn yaml_to_toml() {
    let yaml_input = Format::Toml(parse_toml(MASTER_TOML)).to_yaml();
    let input = Format::Yaml(parse_yaml(&yaml_input));
    let output = input.to_toml();
    let parsed: toml::Value = parse_toml(&output);
    assert_eq!(to_canonical_json(&parse_toml(MASTER_TOML)), to_canonical_json(&parsed));
}

#[test]
fn yaml_to_json() {
    let yaml_input = Format::Toml(parse_toml(MASTER_TOML)).to_yaml();
    let input = Format::Yaml(parse_yaml(&yaml_input));
    let output = input.to_json();
    let parsed: serde_json::Value = parse_json(&output);
    assert_eq!(to_canonical_json(&parse_toml(MASTER_TOML)), parsed);
}

#[test]
fn yaml_to_yaml() {
    let yaml_input = Format::Toml(parse_toml(MASTER_TOML)).to_yaml();
    let input = Format::Yaml(parse_yaml(&yaml_input));
    let output = input.to_yaml();
    let parsed: serde_yml::Value = parse_yaml(&output);
    assert_eq!(to_canonical_json(&parse_toml(MASTER_TOML)), to_canonical_json(&parsed));
}

#[test]
fn roundtrip_toml_json_toml() {
    let original = Format::Toml(parse_toml(MASTER_TOML));
    let as_json = original.to_json();
    let from_json = Format::Json(parse_json(&as_json));
    let back_to_toml = from_json.to_toml();
    let parsed: toml::Value = parse_toml(&back_to_toml);
    assert_eq!(to_canonical_json(&parse_toml(MASTER_TOML)), to_canonical_json(&parsed));
}

#[test]
fn roundtrip_toml_yaml_toml() {
    let original = Format::Toml(parse_toml(MASTER_TOML));
    let as_yaml = original.to_yaml();
    let from_yaml = Format::Yaml(parse_yaml(&as_yaml));
    let back_to_toml = from_yaml.to_toml();
    let parsed: toml::Value = parse_toml(&back_to_toml);
    assert_eq!(to_canonical_json(&parse_toml(MASTER_TOML)), to_canonical_json(&parsed));
}

#[test]
fn roundtrip_json_yaml_json() {
    let json_input = Format::Toml(parse_toml(MASTER_TOML)).to_json();
    let original = Format::Json(parse_json(&json_input));
    let as_yaml = original.to_yaml();
    let from_yaml = Format::Yaml(parse_yaml(&as_yaml));
    let back_to_json = from_yaml.to_json();
    let parsed: serde_json::Value = parse_json(&back_to_json);
    assert_eq!(to_canonical_json(&parse_toml(MASTER_TOML)), parsed);
}
