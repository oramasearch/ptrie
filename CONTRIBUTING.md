# 🧑‍💻 Contributing

The usual process to make a contribution is to:

1. Check for existing related [issues on GitHub](https://github.com/vemonet/ptrie/issues)
2. [Fork](https://github.com/vemonet/ptrie/fork) the repository and create a new branch
3. Make your changes
4. Make sure formatting, linting and tests passes.
5. Add tests if possible to cover the lines you added.
6. Commit, and send a Pull Request.

## 🛠️ Development

Install dependencies:

```bash
rustup update
rustup toolchain install nightly
rustup component add rustfmt clippy
cargo install cargo-tarpaulin git-cliff cargo-outdated
pipx install pre-commit
pre-commit install
```

### 🧪 Tests

Run tests:

```bash
cargo test --all-features
```

Tests with coverage:

```bash
cargo tarpaulin -p ptrie --doc --tests --all-features --out html
```

> Start web server for the cov report: `python -m http.server`

### 🧹 Format and lint

```bash
cargo fmt
cargo clippy --all --all-targets --all-features
```

### 📚 Docs

Generate docs locally:

```bash
cargo doc --all --all-features
```

> Start web server for the generated docs: `python -m http.server --directory target/doc`

### ⏱️ Benchmark

Running benchmarks requires to enable rust nightly: `rustup override set nightly`

```bash
cargo bench
```

### 🏔️ Update changelog

```bash
git cliff -o CHANGELOG.md
```

## 🏷️ New release

Publishing artifacts will be done by the `build.yml` workflow, make sure you have set the following tokens as secrets for this repository: `CRATES_IO_TOKEN`, `CODECOV_TOKEN`

1. Make sure dependencies have been updated:

   ```bash
   cargo update
   cargo outdated
   ```

2. Script to automatically bump the version, update the changelog, then create and push a new tag to GitHub:

   ```bash
   ./.github/release.sh 0.5.0
   ```

3. The `release.yml` workflow will automatically create the release on GitHub, and publish to crates.io.
