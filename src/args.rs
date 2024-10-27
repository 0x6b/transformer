use clap::Parser;

use crate::{command::Command, format::Format};

#[derive(Parser, Debug)]
#[clap(about, version)]
pub struct Args {
    /// Optional input format. Supported format: json, toml, or yaml. If not provided, the format
    /// will be guessed.
    #[arg(short, long)]
    pub from: Option<Format>,

    // Optional output format. Supported format: json, toml, or yaml.
    #[clap(subcommand)]
    pub to: Option<Command>,
}
