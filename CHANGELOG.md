# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.6] - 2026-06-23

### Changed

- Renamed root npm package to `@mauricioleon54/depclerk` (scoped to avoid npm name conflicts)
- CLI command is now `depclerk`

## [0.1.5] - 2026-06-23

### Changed

- Version bump to align with release pipeline fixes

## [0.1.4] - 2026-06-23

### Changed

- Renamed root npm package to `pkgpilot` (previous names taken on npm)
- CLI command is now `pkgpilot` when installed via npm

## [0.1.3] - 2026-06-23

### Changed

- Renamed Windows npm package from `deppilot-win32-x64` to `deppilot-windows-x64` to avoid npm spam detection

## [0.1.2] - 2025-06-23

### Changed

- Updated release secrets configuration

## [0.1.0] - 2025-06-23

### Added

- Automated release pipeline: GitHub Releases, crates.io, npm, Homebrew tap
- npm distribution via platform-specific optional dependencies (same pattern as esbuild)
- `scripts/bump-version` — atomic version bump across Cargo.toml + all npm packages
- `scripts/check-release` — pre-release sanity checker (git state, version sync, tests, dry-run)
- `homebrew/deppilot.rb.template` — Homebrew formula template filled by CI
- `RELEASING.md` — full release guide with secrets, recovery procedures, and rollback instructions
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

[Unreleased]: https://github.com/MauricioLeon54/DepPilot/compare/v0.1.6...HEAD
[0.1.6]: https://github.com/MauricioLeon54/DepPilot/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/MauricioLeon54/DepPilot/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/MauricioLeon54/DepPilot/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/MauricioLeon54/DepPilot/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/MauricioLeon54/DepPilot/compare/v0.1.0...v0.1.2
[0.1.0]: https://github.com/MauricioLeon54/DepPilot/releases/tag/v0.1.0
