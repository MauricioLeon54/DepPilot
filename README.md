# DepPilot

**Dependency update assistant for JavaScript/TypeScript projects.**

DepPilot updates your npm/yarn/pnpm dependencies one at a time and creates a clean git commit for each — so your history stays readable, bisectable, and reviewable.

---

## Installation

### npm / yarn / pnpm (no Rust required)

```bash
npm install -g deppilot
yarn global add deppilot
pnpm add -g deppilot
```

### npx (no install)

```bash
npx deppilot update
```

### Homebrew (macOS and Linux)

```bash
brew tap mauuleo/deppilot
brew install deppilot
```

Or in one command:

```bash
brew install mauuleo/deppilot/deppilot
```

### Cargo (requires Rust)

```bash
cargo install deppilot
```

### Pre-built binary

Download from [GitHub Releases](https://github.com/mauuleo/deppilot/releases):

| Platform | File |
|---|---|
| macOS Apple Silicon | `deppilot-macos-aarch64.tar.gz` |
| macOS Intel | `deppilot-macos-x86_64.tar.gz` |
| Linux x86\_64 (static musl) | `deppilot-linux-x86_64.tar.gz` |
| Linux arm64 | `deppilot-linux-aarch64.tar.gz` |
| Windows x64 | `deppilot-windows-x86_64.zip` |

```bash
# Example: macOS Apple Silicon
curl -LO https://github.com/mauuleo/deppilot/releases/latest/download/deppilot-macos-aarch64.tar.gz
tar -xzf deppilot-macos-aarch64.tar.gz
chmod +x deppilot
sudo mv deppilot /usr/local/bin/
```

Checksums are published in `SHA256SUMS.txt` alongside each release.

---

## Quick start

```bash
cd my-project          # navigate to a JS/TS project
deppilot update        # update all dependencies, one commit each
```

DepPilot will:

1. Find your `package.json`
2. Detect your package manager (`yarn.lock` → Yarn, `pnpm-lock.yaml` → pnpm, `package-lock.json` → npm)
3. Update each dependency using the correct install command
4. Show the generated `git commit` command and let you edit the message
5. Commit only `package.json` + the lock file — nothing else

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

### Update to a specific version

```bash
deppilot update axios@^1.18.1
```

### Update multiple packages (one commit each)

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
| `--only deps\|dev` | Restrict to production or dev dependencies |
| `-y, --yes` | Accept all prompts (CI-friendly) |
| `--no-commit` | Update without committing |
| `--dry-run` | Print what would happen without changing files |
| `--check <CMD>` | Shell command to validate each update |
| `--continue-on-error` | Keep updating even if validation fails |
| `--force` | Continue even if the package manager exits non-zero |
| `--commit-template <TPL>` | Custom commit message template |
| `--package-manager yarn\|pnpm\|npm` | Override auto-detection |

---

## Examples

### Dry run — preview everything, change nothing

```bash
deppilot update --dry-run
```

### CI — update all, auto-accept, no commits

```bash
deppilot update --yes --no-commit
```

### CI — update all, auto-commit with template

```bash
deppilot update --yes --commit-template "chore(deps): update {name} to {version}"
```

### Validate after each update, stop on failure

```bash
deppilot update --check "yarn lint && yarn tsc --noEmit"
```

### Validate but keep going on failure

```bash
deppilot update --check "yarn test" --continue-on-error
```

### Custom commit message template

```bash
deppilot update --commit-template "build: bump {name} to {version}"
```

Available placeholders: `{name}` (package name), `{version}` (target version)

---

## Package manager examples

### yarn

```bash
deppilot update                    # auto-detects yarn.lock
deppilot update axios@^1.18.1     # yarn add axios@^1.18.1
deppilot update jest --only dev   # yarn add jest@latest --dev
```

### pnpm

```bash
deppilot update                    # auto-detects pnpm-lock.yaml
deppilot update axios@^1.18.1     # pnpm add axios@^1.18.1
deppilot update jest --only dev   # pnpm add jest@latest -D
```

### npm

```bash
deppilot update                    # auto-detects package-lock.json
deppilot update axios@^1.18.1     # npm install axios@^1.18.1 --save
deppilot update jest --only dev   # npm install jest@latest --save-dev
```

---

## Git safety

- Only `package.json` and the lock file are ever staged
- Unrelated working-tree changes trigger a warning and confirmation before proceeding
- `--force` skips the confirmation; `--yes` auto-confirms

---

## How npm distribution works

The `deppilot` npm package uses **optional dependencies** to distribute the correct native binary per platform:

```
deppilot (root, contains JS wrapper)
├── deppilot-darwin-arm64   macOS Apple Silicon
├── deppilot-darwin-x64     macOS Intel
├── deppilot-linux-x64      Linux x86_64 (musl, statically linked)
├── deppilot-linux-arm64    Linux arm64
└── deppilot-win32-x64      Windows x64
```

npm/yarn/pnpm automatically install only the package matching your platform. The JS wrapper in `bin/deppilot.js` locates and exec-spawns the native binary. Zero startup overhead.

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Releasing

See [RELEASING.md](RELEASING.md) for the full release process, required secrets, and recovery procedures.

## License

MIT — see [LICENSE](LICENSE).
