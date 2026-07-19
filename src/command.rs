use clap::Parser;

#[derive(Parser, Debug)]
pub enum Command {
    /// Convert the input to JSON. This is the default if no subcommand is provided.
    Json,
    /// Convert the input to TOML
    Toml,
    /// Convert the input to YAML
    Yaml,
    /// Convert the input to Go type definitions
    Go {
        /// Root Go type name
        #[arg(long = "type", value_name = "NAME")]
        type_name: Option<String>,

        /// Emit anonymous nested structs
        #[arg(long = "inline", action = clap::ArgAction::SetFalse, default_value_t = true)]
        flatten: bool,

        /// Add example tags for scalar values
        #[arg(long)]
        example: bool,

        /// Add omitempty to every JSON tag
        #[arg(long = "omitempty")]
        all_omitempty: bool,
    },
    /// Convert the input to Rust serde struct
    Serde {
        /// The name of the root struct to generate
        #[arg(short, long, default_value = "Root")]
        name: String,

        /// Derives to add to the generated struct
        #[arg(short, long, default_value = "Debug, Serialize, Deserialize")]
        derives: String,
    },
}
