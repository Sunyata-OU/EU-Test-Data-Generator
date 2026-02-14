# Contributing

See the full [CONTRIBUTING.md](https://github.com/Sunyata-OU/idsmith/blob/main/CONTRIBUTING.md) on GitHub.

## Quick Reference

### Core Rust Crate

```bash
cargo test -p idsmith
cargo fmt -p idsmith -- --check
cargo clippy -p idsmith -- -D warnings
```

### Python Bindings

```bash
cd bindings/python
pip install maturin[patchelf] pytest
maturin develop
pytest tests/ -v
```

### Node.js Bindings

```bash
cd bindings/node
npm install
npm run build
npm test
```

## Repo Structure

```
idsmith/
├── src/                   # Rust core library + CLI
├── tests/                 # Integration tests
├── bindings/
│   ├── python/            # PyO3 + maturin → PyPI
│   └── node/              # napi-rs → npm
├── docs/                  # This documentation (mdBook)
└── .github/workflows/     # CI per package
```
