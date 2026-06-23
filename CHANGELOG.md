# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-06-23

### Added

- `deppilot update` command — update all or specific dependencies one by one
- Auto-detection of project root by walking up to the nearest `package.json`
- Auto-detection of package manager via lock files (`yarn.lock`, `pnpm-lock.yaml`, `package-lock.json`)
- Interactive prompt when multiple lock files are found
- Per-dependency git commit workflow with editable commit messages
- `--only deps` / `--only dev` filter to restrict which section is updated
- `--yes` flag to skip all confirmation prompts (CI-friendly)
- `--no-commit` flag to update without committing
- `--dry-run` flag to preview all actions without modifying files
- `--check` flag to run a validation command after each update
- `--continue-on-error` to keep going when validation fails
- `--force` to continue when the package manager exits non-zero
- `--commit-template` for custom commit message templates with `{name}` and `{version}` placeholders
- `--package-manager` to override auto-detection
- Unrelated working-tree change detection with user confirmation
- Beautiful terminal output with colors, icons, spinner, and summary
- 29 unit tests covering detection, parsing, and command generation

[Unreleased]: https://github.com/mauuleo/deppilot/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/mauuleo/deppilot/releases/tag/v0.1.0
