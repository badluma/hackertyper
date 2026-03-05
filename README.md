# hackertyper

A local, customizable CLI alternative for [hackertyper.net](https://hackertyper.net). Type anything and watch the output stream across your terminal character by character.

---

## Install

### Prerequisites

You need Rust and Cargo. If you don't have them, grab Rust from [rustup.rs](https://rustup.rs):

```bash
# Linux / macOS / Windows Subsystem for Linux (WSL)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
or install them with brew:
```bash
brew install rust
```

Make sure you're running **Rust 1.85 or later**. Check your version:

```bash
rustc --version
```

Update rust with:
```bash
rustup update stable
```

### Quick Install (Recommended)

```bash
cargo install hackertyper
```

This grabs the latest crate from crates.io and installs it to `~/.cargo/bin/hackertyper`. Make sure `~/.cargo/bin` is on your `PATH` (rustup handles this automatically in most cases).

### Build from Source

```bash
git clone https://github.com/badluma/hackertyper
cd hackertyper
cargo install --path .
```

---

## Platform Support

**Linux & macOS:** Fully supported.

**Windows:** Not directly supported due to the `termios` dependency. However, you can use **Windows Subsystem for Linux (WSL2)** to run hackertyper on Windows. Install WSL2, then follow the Linux installation steps above.

---

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

`light_black` `light_red` `light_green` `light_yellow` `light_blue` `light_magenta` `light_cyan` `light_white`

## Examples

```bash
# Type out a file at default speed
hackertyper -p /path/to/file

# Faster output in green, looping
hackertyper -p /path/to/file -s 8 -c green -l

# Slow, dramatic red output
hackertyper -p /path/to/file -s 1 -c red
```

## License

MIT