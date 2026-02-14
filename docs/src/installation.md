# Installation

## Rust

Add to your `Cargo.toml`:

```toml
[dependencies]
idsmith = { version = "0.4.0", default-features = false }
```

Or install the CLI:

```bash
cargo install idsmith
```

> **Note:** Use `default-features = false` when using as a library to exclude CLI dependencies (clap, etc.).

## Python

Requires Python 3.8+.

```bash
pip install idsmith
```

Pre-built wheels are available for Linux, macOS, and Windows.

## Node.js

Requires Node.js 18+.

```bash
npm install idsmith
```

Pre-built native binaries are available for:
- Linux (x86_64, aarch64)
- macOS (x86_64, aarch64)
- Windows (x86_64)

## From Source

```bash
git clone https://github.com/Sunyata-OU/idsmith
cd idsmith

# Rust
cargo build --release

# Python
cd bindings/python
pip install maturin
maturin develop

# Node.js
cd bindings/node
npm install
npm run build
```
