# transformer

A CLI tool to convert between JSON, TOML, and YAML.

I'm an avid user of https://transform.tools/ to convert between JSON, TOML, and YAML from time to time. However, I wanted a CLI tool that could do the same thing offline, saving me the hassle of copy-pasting.

## Usage

```console
$ transformer --help
A CLI tool to convert between JSON, TOML, and YAML, reading from stdin.

Usage: transformer [OPTIONS]

Options:
  -f, --from <FROM>  Optional input format. Supported format: json, toml, or yaml. If
                     not provided, the format will be guessed
  -t, --to <TO>      Optional output format. Supported format: json, toml, or yaml
                     [default: json]
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
