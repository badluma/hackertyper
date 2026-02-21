<img width="1000" height="262" alt="banner" src="https://github.com/user-attachments/assets/9dd2403e-26fb-48cf-aad5-68ea7cb3e7bf" />

A local, customizable CLI alternative for [hackertyper.net](https://hackertyper.net). Type anything — output any file, character by character.

---

## Install

**Platform:** Linux and macOS only. Windows is not supported (`termios` dependency).

### Prerequisites

You need Rust and Cargo. If you don't have them, install via [rustup](https://rustup.rs):

```bash
# Linux / macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This project requires **Rust 1.85 or later** (Cargo edition 2024). Verify your version:

```bash
rustc --version
```

To update an existing installation:

```bash
rustup update stable
```

### Building from source

```bash
git clone https://github.com/badluma/hackertyper
cd hackertyper
cargo install --path .
```

The binary is installed to `~/.cargo/bin/hackertyper`. Make sure `~/.cargo/bin` is on your `PATH` — rustup adds this automatically, but if it isn't:

```bash
# Add to ~/.bashrc, ~/.zshrc, or equivalent
export PATH="$HOME/.cargo/bin:$PATH"
```

## Usage

```bash
hackertyper --path <file>
```

Press any key to advance the output. Press `Ctrl+C` to exit.

## Options

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--path` | `-p` | required | Path to the file to type out |
| `--speed` | `-s` | `4` | Characters printed per keypress |
| `--color` | `-c` | `default` | Output color |
| `--loop` | `-l` | `false` | Loop the file continuously |

## Colors

`black` `red` `green` `yellow` `blue` `magenta` `cyan` `white`

`bright_black` `bright_red` `bright_green` `bright_yellow` `bright_blue` `bright_magenta` `bright_cyan` `bright_white`

## Examples

```bash
# Type out a file at default speed
hackertyper -p /path/to/file

# Faster output in green, looping
hackertyper -p /path/to/file -s 8 -c green -l

# Slow, dramatic red output
hackertyper -p /path/to/file 1 -c bright_red
```

## License

MIT
