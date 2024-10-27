# transformer

A CLI tool to convert from JSON|TOML|YAML to JSON|TOML|YAML|Rust serde structs.

I'm an avid user of https://transform.tools/ to convert between JSON, TOML, YAML, and Rust serde structs from time to time. However, I wanted a CLI tool that could do the same thing offline, saving me the hassle of copy-pasting.

## Usage

```console
A CLI tool to convert from JSON|TOML|YAML to JSON|TOML|YAML|Rust serde structs.

Usage: transformer [OPTIONS] [COMMAND]

Commands:
  json   Convert the input to JSON. This is the default if no subcommand is provided
  toml   Convert the input to TOML
  yaml   Convert the input to YAML
  serde  Convert the input to Rust serde struct
  help   Print this message or the help of the given subcommand(s)

Options:
  -f, --from <FROM>  Optional input format. Supported format: json, toml, or yaml. If
                     not provided, the format will be guessed
  -h, --help         Print help
  -V, --version      Print version
```

i.e.

```console
$ # gives you a JSON equivalent of the input
$ cat Cargo.toml | transformer
```

## Contributing

There should be similar and/or capable tools available in every language, so if you have a better option, feel free to keep using it. I wrote this one for quick access. While I don't expect any issues or pull requests, you're welcome to fork it and modify it as you see fit.

## License

MIT. See [LICENSE](LICENSE) for details.
