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

### Install from GitHub with Cargo

```sh
cargo install --git https://github.com/c0dex-erased/rfetch.git
```

Cargo installs the `rfetch` executable into `~/.cargo/bin`, so it can be run from any working directory. Make sure `~/.cargo/bin` is on your `PATH`. If your shell cannot find `rfetch`, add this to your shell startup file:

```sh
export PATH="$HOME/.cargo/bin:$PATH"
```

Then open a new shell, or load the file you edited (for example, `source ~/.bashrc` or `source ~/.zshrc`). Verify the installation with `rfetch --version`. Update later with the same `cargo install --git ...` command.

### Build from a local clone

```sh
git clone https://github.com/c0dex-erased/rfetch.git
cd rfetch
cargo build --release
```

The built-in logos are embedded into the executable, so the binary works without the source checkout or a separate `logos/` directory.

Configuration is read from `~/.config/rfetch/config.toml`. Missing config uses defaults. Invalid TOML is reported with a useful parse error. `--gen-config` creates the directory and writes a default config.

The config supports `[general]`, `[logo]`, `[display]`, and `[modules]`. Logo `type` can be `auto` or a built-in name; set `path` to a text file for a custom logo. Auto selection checks exact `ID` before each `ID_LIKE` value, then falls back to `default.txt`.

## Supported distributions

Built-in distribution logos are available for:

- Alpine Linux
- Arch Linux
- Debian
- Fedora
- Gentoo
- openSUSE (including Leap and Tumbleweed)
- Ubuntu
- Void Linux

Other distributions use the generic Linux penguin logo, adapted from pfetch.

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
