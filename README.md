# rfetch

Minimalist Linux system information CLI written in Rust. Version 0.1.0 targets Linux x86_64 and uses `/etc/os-release`, `/proc`, and environment variables; it does not invoke shell commands or access the network at runtime.

## Build and run

```sh
cargo run
cargo run -- --logo gentoo
cargo run -- --logo auto --no-color
cargo run -- --gen-config
cargo build --release
```

Install the release binary by copying `target/release/rfetch` into a directory on your `PATH`. Logo files are loaded from the repository's `logos/` directory, so keep that directory alongside the source checkout when running with this build.

Configuration is read from `~/.config/rfetch/config.toml`. Missing config uses defaults. Invalid TOML is reported with a useful parse error. `--gen-config` creates the directory and writes a default config.

The config supports `[general]`, `[logo]`, `[display]`, and `[modules]`. Logo `type` can be `auto` or a built-in name; set `path` to a text file for a custom logo. Auto selection checks exact `ID` before each `ID_LIKE` value, then falls back to `default.txt`.

## Architecture

- `cli`: parses the small command-line interface.
- `config`: TOML schema, defaults, and config file handling.
- `system`: reads Linux system files.
- `logo`: logo selection and file loading.
- `colors`: ANSI color formatting.
- `modules`: assembles the display.
- `main`: reports errors without panicking.

Released under GPL-3.0; see [LICENSE](LICENSE).

The ASCII distribution logos are adapted from [dylanaraps/pfetch](https://github.com/dylanaraps/pfetch), licensed under GPL-3.0.
