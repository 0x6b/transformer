use serde::Serialize;
use serde_json::{to_string_pretty as json_to_string, Value as JsonValue};
use serde_yml::{to_string as yaml_to_string, Value as YamlValue};
use toml::{to_string_pretty as toml_to_string, Value as TomlValue};

#[derive(Debug, Clone, Serialize)]
pub enum Format {
    Json(JsonValue),
    Toml(TomlValue),
    Yaml(YamlValue),
}

impl From<&str> for Format {
    fn from(s: &str) -> Self {
        match s.to_lowercase().chars().next() {
            Some('j') => Format::Json(JsonValue::Null),
            Some('t') => Format::Toml(TomlValue::String(String::new())),
            Some('y') => Format::Yaml(YamlValue::Null),
            _ => panic!("Unknown format: {s}"),
        }
    }
}

impl Format {
    pub fn to_json(&self) -> String {
        match self {
            Format::Json(v) => json_to_string(&v).unwrap(),
            Format::Toml(v) => json_to_string(&v).unwrap(),
            Format::Yaml(v) => json_to_string(&v).unwrap(),
        }
    }

    pub fn to_toml(&self) -> String {
        match self {
            Format::Json(v) => toml_to_string(&v).unwrap(),
            Format::Toml(v) => toml_to_string(&v).unwrap(),
            Format::Yaml(v) => toml_to_string(&v).unwrap(),
        }
    }

    pub fn to_yaml(&self) -> String {
        match self {
            Format::Json(v) => yaml_to_string(&v).unwrap(),
            Format::Toml(v) => yaml_to_string(&v).unwrap(),
            Format::Yaml(v) => yaml_to_string(&v).unwrap(),
        }
    }
}
