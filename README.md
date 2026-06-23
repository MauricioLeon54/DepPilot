# DepPilot

**Dependency update assistant for JavaScript/TypeScript projects.**

DepPilot updates your npm/yarn/pnpm dependencies one at a time and creates a clean git commit for each update — so your history stays readable, bisectable, and reviewable.

---

## Installation

### Cargo (recommended)

```bash
cargo install deppilot
```

### Homebrew (coming soon)

```bash
brew tap mauuleo/tap
brew install deppilot
```

### Pre-built binaries

Download from the [GitHub Releases](https://github.com/mauuleo/deppilot/releases) page.
Binaries are available for:

| Platform | File |
|---|---|
| Linux x86\_64 | `deppilot-linux-x86_64` |
| Linux arm64 | `deppilot-linux-aarch64` |
| Linux x86\_64 musl (static) | `deppilot-linux-x86_64-musl` |
| macOS x86\_64 | `deppilot-macos-x86_64` |
| macOS arm64 (Apple Silicon) | `deppilot-macos-aarch64` |
| Windows x86\_64 | `deppilot-windows-x86_64.exe` |

### npm wrapper (optional)

If you prefer installing via npm in a JS project:

```bash
npm install -g deppilot
```

> The npm wrapper downloads the appropriate pre-built binary for your platform on first run.

---

## Quick start

```bash
cd my-project          # navigate to a JS/TS project
deppilot update        # update all dependencies, one commit each
```

DepPilot will:

1. Detect your project root (`package.json`)
2. Detect your package manager (`yarn.lock` → Yarn, `pnpm-lock.yaml` → pnpm, `package-lock.json` → npm)
3. Update each dependency using the correct install command
4. Show you the generated `git commit` command
5. Let you edit the commit message or press Enter to accept
6. Commit only `package.json` + the lock file — nothing else

---

## Usage

```
deppilot update [PACKAGES...] [OPTIONS]
```

### Update all dependencies

```bash
deppilot update
```

### Update a single package

```bash
deppilot update axios
```

### Update a package to a specific version

```bash
deppilot update axios@^1.18.1
```

### Update multiple packages at once (one commit each)

```bash
deppilot update axios vue firebase
deppilot update axios@^1.18.1 vue@^3.5.38
```

### Update only devDependencies

```bash
deppilot update --only dev
```

### Update only production dependencies

```bash
deppilot update --only deps
```

---

## Options

| Flag | Description |
|---|---|
| `--only deps\|dev` | Restrict to production (`deps`) or dev (`dev`) dependencies |
| `-y, --yes` | Accept all prompts automatically |
| `--no-commit` | Update without creating git commits |
| `--dry-run` | Print what would happen without changing any files |
| `--check <COMMAND>` | Shell command to validate each update (e.g. `yarn lint`) |
| `--continue-on-error` | Keep updating even if validation fails |
| `--force` | Continue even when the package manager exits non-zero |
| `--commit-template <TEMPLATE>` | Custom commit message template (default: `chore: update {name}`) |
| `--package-manager yarn\|pnpm\|npm` | Override package manager auto-detection |

---

## Examples

### Dry run — preview everything, change nothing

```bash
deppilot update --dry-run
```

Output example:

```
 DepPilot
 Dependency Update Assistant

  Project  /Users/me/my-app
  Package  my-app
  Manager  🧶 yarn

  Updating 3 dependencies

  ↑  axios  dep  → latest
  [dry-run]  yarn add axios@latest
  ↑  react  dep  → latest
  [dry-run]  yarn add react@latest
  ↑  eslint  dev  → latest
  [dry-run]  yarn add eslint@latest --dev
```

### CI / automation — no prompts, no commits

```bash
deppilot update --yes --no-commit
```

### CI — update everything and auto-commit

```bash
deppilot update --yes --commit-template "chore(deps): update {name} to {version}"
```

### Run lint + type-check after each update, stop on failure

```bash
deppilot update --check "yarn lint && yarn tsc --noEmit"
```

### Run validation but keep going even if it fails

```bash
deppilot update --check "yarn test" --continue-on-error
```

### Custom commit template

```bash
deppilot update --commit-template "chore(deps): update {name}"
deppilot update --commit-template "build: bump {name} to {version}"
```

Available placeholders:

| Placeholder | Value |
|---|---|
| `{name}` | Package name (e.g. `axios`) |
| `{version}` | Target version (e.g. `^1.18.1` or `latest`) |

### With yarn

```bash
deppilot update                          # detects yarn.lock automatically
deppilot update axios@^1.18.1           # yarn add axios@^1.18.1
deppilot update jest --only dev         # yarn add jest@latest --dev
```

### With pnpm

```bash
deppilot update                          # detects pnpm-lock.yaml automatically
deppilot update axios@^1.18.1           # pnpm add axios@^1.18.1
deppilot update jest --only dev         # pnpm add jest@latest -D
```

### With npm

```bash
deppilot update                          # detects package-lock.json automatically
deppilot update axios@^1.18.1           # npm install axios@^1.18.1 --save
deppilot update jest --only dev         # npm install jest@latest --save-dev
```

### Override package manager

```bash
deppilot update --package-manager pnpm
```

---

## Git safety

DepPilot only ever stages `package.json` and your lock file. It will never commit unrelated changes.

Before starting, if your working tree has files outside of those two, DepPilot warns you and asks for confirmation (or respects `--force` / `--yes`).

---

## Publishing strategy

| Method | Command | Best for |
|---|---|---|
| **crates.io** | `cargo install deppilot` | Rust developers |
| **GitHub Releases** | Download binary | Everyone else |
| **Homebrew** | `brew install deppilot` | macOS users |
| **npm wrapper** | `npm install -g deppilot` | JS teams who prefer npm |

For automated releases, push a version tag — the CI workflow builds all platform binaries, creates a GitHub Release, and publishes to crates.io in one step:

```bash
git tag v0.2.0
git push origin v0.2.0
```

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT — see [LICENSE](LICENSE).
