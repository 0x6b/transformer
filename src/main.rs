use anyhow::Result;
use clap::Parser;
use serde::Serialize;
use serde_json::{
    from_str as json_from_str, to_string_pretty as json_to_string, Value as JsonValue,
};
use serde_yml::{from_str as yaml_from_str, to_string as yaml_to_string, Value as YamlValue};
use std::io::{stdin, Read};
use toml::{from_str as toml_from_str, to_string_pretty as toml_to_string, Value as TomlValue};

use crate::Format::{Json, Toml, Yaml};

#[derive(Debug, Clone, Serialize)]
enum Format {
    Json(JsonValue),
    Toml(TomlValue),
    Yaml(YamlValue),
}

impl From<&str> for Format {
    fn from(s: &str) -> Self {
        match s.to_lowercase().chars().next() {
            Some('j') => Json(JsonValue::Null),
            Some('t') => Toml(TomlValue::String(String::new())),
            Some('y') => Yaml(YamlValue::Null),
            _ => panic!("Unknown format: {s}"),
        }
    }
}

impl Format {
    fn to_json(&self) -> String {
        match self {
            Json(v) => json_to_string(&v).unwrap(),
            Toml(v) => json_to_string(&v).unwrap(),
            Yaml(v) => json_to_string(&v).unwrap(),
        }
    }

    fn to_toml(&self) -> String {
        match self {
            Json(v) => toml_to_string(&v).unwrap(),
            Toml(v) => toml_to_string(&v).unwrap(),
            Yaml(v) => toml_to_string(&v).unwrap(),
        }
    }

    fn to_yaml(&self) -> String {
        match self {
            Json(v) => yaml_to_string(&v).unwrap(),
            Toml(v) => yaml_to_string(&v).unwrap(),
            Yaml(v) => yaml_to_string(&v).unwrap(),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(about, version)]
struct Args {
    /// Optional input format. Supported format: json, toml, or yaml. If not provided, the format
    /// will be guessed.
    #[arg(short, long)]
    from: Option<Format>,

    /// Optional output format. Supported format: json, toml, or yaml
    #[arg(short, long, default_value = "json")]
    to: Format,
}

fn main() -> Result<()> {
    let Args { from, to } = Args::parse();
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let input = match from {
        None => guess(&input),
        Some(f) => match f {
            Json(_) => Json(json_from_str(&input)?),
            Toml(_) => Toml(toml_from_str(&input)?),
            Yaml(_) => Yaml(yaml_from_str(&input)?),
        },
    };

    println!(
        "{}",
        match to {
            Json(_) => input.to_json(),
            Toml(_) => input.to_toml(),
            Yaml(_) => input.to_yaml(),
        }
    );

    Ok(())
}

fn guess(input: &str) -> Format {
    if let Ok(value) = json_from_str(input) {
        Json(value)
    } else if let Ok(value) = toml_from_str(input) {
        Toml(value)
    } else if let Ok(value) = yaml_from_str(input) {
        Yaml(value)
    } else {
        panic!("Unknown format")
    }
}
