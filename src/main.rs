mod args;
mod command;
mod format;

use std::io::{stdin, Read};

use anyhow::{anyhow, Result};
use clap::Parser;
use json_typegen_shared::{codegen, ImportStyle, Options};
use serde_json::from_str as json_from_str;
use serde_yml::from_str as yaml_from_str;
use toml::from_str as toml_from_str;

use crate::{
    args::Args,
    command::Command,
    format::Format,
    Format::{Json, Toml, Yaml},
};

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

    match to {
        None | Some(Command::Json) => println!("{}", input.to_json()),
        Some(Command::Toml) => println!("{}", input.to_toml()),
        Some(Command::Yaml) => println!("{}", input.to_yaml()),
        Some(Command::Serde { name, derives }) => {
            let mut options = Options::default();
            options.derives = derives;
            options.import_style = ImportStyle::AssumeExisting;
            let result = codegen(&name, &input.to_json(), options).map_err(|e| anyhow!("{e}"))?;
            println!("{result}");
        }
    }

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
