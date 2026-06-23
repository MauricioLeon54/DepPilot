# Contributing to DepPilot

Thank you for your interest in contributing! This document covers everything you need to get started.

## Prerequisites

- [Rust](https://rustup.rs/) stable toolchain (1.75+)
- `cargo` available in your PATH
- `git`

## Setup

```bash
git clone https://github.com/mauuleo/deppilot.git
cd deppilot
cargo build
```

## Running tests

```bash
cargo test
```

All 29 tests should pass. Tests are colocated with the code they cover using `#[cfg(test)]` modules.

## Code style

```bash
cargo fmt        # Format code
cargo clippy     # Lint
```

CI enforces both. Your PR must pass `cargo fmt --check` and `cargo clippy -- -D warnings`.

## Project structure

| Module | Responsibility |
|---|---|
| `cli` | Clap argument structs |
| `errors` | `DepPilotError` domain error enum |
| `project` | Find the project root (nearest `package.json`) |
| `package_json` | Parse dependencies and devDependencies |
| `package_manager` | Detect PM, generate install commands |
| `git` | Stage files, commit, check working tree |
| `update` | Orchestrate the full update loop |
| `prompt` | Interactive prompts (confirm, edit, select) |
| `validation` | Run user-supplied check commands |
| `output` | Terminal output, colors, spinner, summary |

## Adding a feature

1. Open an issue first to discuss the change
2. Create a branch from `main`
3. Write tests alongside the code
4. Update `CHANGELOG.md` under `[Unreleased]`
5. Open a pull request

## Commit style

```
type: short description

Optional body explaining the why.
```

Types: `feat`, `fix`, `chore`, `docs`, `test`, `refactor`, `ci`

## Releasing

Releases are fully automated. Push a version tag:

```bash
git tag v0.2.0
git push origin v0.2.0
```

The release workflow builds cross-platform binaries, creates a GitHub Release, and publishes to crates.io.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
