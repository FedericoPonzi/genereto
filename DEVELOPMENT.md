# Development Guide

## Requirements

- Rust (latest stable)
- Git

## Development Setup

```bash
git clone https://github.com/FedericoPonzi/genereto.git
cd genereto
cargo build
cargo test
```

## Publishing a New Release

1. Update version in `Cargo.toml`:
   ```toml
   [package]
   version = "x.y.z"
   ```

2. Commit and push changes:
   ```bash
   git add Cargo.toml
   git commit -m "Bump version to x.y.z"
   git push origin main
   ```

3. Create and push tag:
   ```bash
   git tag vx.y.z
   git push origin vx.y.z
   ```

The GitHub Actions workflow will automatically build the binary and create a release.