# DepPilot

> Update your JS/TS dependencies one at a time — with clean git commits for each.

Keeping dependencies up to date usually means a single giant commit that says "chore: bump deps" and is impossible to bisect or review. DepPilot fixes that. It updates each package individually, shows you the install command it will run, and creates a focused git commit you can actually understand later.

---

## Why DepPilot?

- **One commit per dependency.** Your git history stays readable. `git bisect` actually works.
- **Edit commit messages.** The generated message is pre-filled — press Enter to accept or type to customize.
- **Safe by design.** DepPilot only stages `package.json` and your lock file. It never touches unrelated files.
- **Zero config.** Drop it into any project. It auto-detects your package manager from the lock file.
- **Validate as you go.** Run lint, tests, or type-check after each update and stop early if something breaks.

---

## Install

### npm, yarn, or pnpm

```bash
npm install -g deppilot
yarn global add deppilot
pnpm add -g deppilot
```

### npx — no install needed

```bash
npx deppilot update
```

### Homebrew

```bash
brew install mauuleo/deppilot/deppilot
```

### Cargo

```bash
cargo install deppilot
```

### Pre-built binary

Grab the binary for your platform from the [releases page](https://github.com/mauuleo/deppilot/releases):

| Platform | File |
|---|---|
| macOS (Apple Silicon) | `deppilot-macos-aarch64.tar.gz` |
| macOS (Intel) | `deppilot-macos-x86_64.tar.gz` |
| Linux x86\_64 | `deppilot-linux-x86_64.tar.gz` |
| Linux arm64 | `deppilot-linux-aarch64.tar.gz` |
| Windows x64 | `deppilot-windows-x86_64.zip` |

```bash
# macOS example
curl -LO https://github.com/mauuleo/deppilot/releases/latest/download/deppilot-macos-aarch64.tar.gz
tar -xzf deppilot-macos-aarch64.tar.gz
sudo mv deppilot /usr/local/bin/
```

SHA256 checksums are in `SHA256SUMS.txt` on every release.

---

## Quick start

```bash
cd your-project
deppilot update
```

That's it. DepPilot will:

1. Find your `package.json`
2. Detect your package manager from the lock file
3. Update each dependency, one at a time
4. Show you the git commands it's about to run
5. Let you edit the commit message — or just press Enter to accept

---

## How it works

### Package manager detection

DepPilot looks for a lock file in your project root:

| Lock file | Package manager |
|---|---|
| `yarn.lock` | Yarn |
| `pnpm-lock.yaml` | pnpm |
| `package-lock.json` | npm |

If you have multiple lock files (unusual, but it happens), DepPilot asks which one to use. You can also skip detection entirely with `--package-manager yarn|pnpm|npm`.

### The commit workflow

For each dependency, DepPilot shows you the exact commands it's going to run:

```
  ↑  axios  dep  → latest

  Git commands:
    git add package.json yarn.lock
    git commit -m "chore: update axios"

  Commit message (Enter to accept, clear to skip): chore: update axios
```

Press **Enter** to commit with the generated message, type to change it, or clear the field to skip the commit for that package.

---

## Usage examples

### Update everything

```bash
deppilot update
```

### Update a specific package

```bash
deppilot update axios
```

### Update to a specific version

```bash
deppilot update axios@^1.18.1
```

### Update several packages at once

Each one gets its own commit.

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

### Dry run — see what would happen without changing anything

```bash
deppilot update --dry-run
```

### Skip commit prompts

```bash
deppilot update --yes
```

### Update without committing

```bash
deppilot update --no-commit
```

### Run a validation command after each update

Stops immediately if the command fails, leaving the changes unstaged so you can inspect them.

```bash
deppilot update --check "yarn lint && yarn tsc --noEmit"
```

### Validate but keep going even on failure

```bash
deppilot update --check "yarn test" --continue-on-error
```

### Use a custom commit message template

```bash
deppilot update --commit-template "chore(deps): update {name} to {version}"
```

Available placeholders: `{name}` and `{version}`.

### CI mode — update everything, auto-commit, no prompts

```bash
deppilot update --yes --commit-template "chore(deps): update {name}"
```

### CI mode — update without committing (let your CI handle the commit)

```bash
deppilot update --yes --no-commit
```

---

## Package manager command reference

DepPilot translates each update into the right command for your package manager automatically.

| DepPilot | yarn | pnpm | npm |
|---|---|---|---|
| `update axios` | `yarn add axios@latest` | `pnpm add axios@latest` | `npm install axios@latest --save` |
| `update axios@^1.18.1` | `yarn add axios@^1.18.1` | `pnpm add axios@^1.18.1` | `npm install axios@^1.18.1 --save` |
| `update jest --only dev` | `yarn add jest@latest --dev` | `pnpm add jest@latest -D` | `npm install jest@latest --save-dev` |

---

## All options

| Flag | Default | Description |
|---|---|---|
| `--only deps\|dev` | — | Update only production or dev dependencies |
| `-y, --yes` | false | Accept all prompts automatically |
| `--no-commit` | false | Update packages without committing |
| `--dry-run` | false | Preview actions without changing any files |
| `--check <CMD>` | — | Shell command to run after each update |
| `--continue-on-error` | false | Keep going even if `--check` fails |
| `--force` | false | Continue even if the package manager exits non-zero |
| `--commit-template <TPL>` | `chore: update {name}` | Custom commit message template |
| `--package-manager yarn\|pnpm\|npm` | auto | Override package manager detection |

---

## Safety guarantees

**DepPilot will never commit unrelated files.**

Before staging anything, it checks that `package.json` and the lock file are the only modified files. If your working tree has other changes, it warns you and asks before proceeding. Use `--force` to skip the prompt, or `--no-commit` if you'd rather handle git yourself.

**One dependency per commit, always.**

Each package gets its own `git add` and `git commit`. If you're updating 10 packages and the 7th one breaks validation, you'll have 6 clean commits and a clean workspace to inspect the failure.

**Dry run before you commit.**

Not sure what DepPilot will do in your project? Run `deppilot update --dry-run` first. It prints every command it would run without touching anything.

---

## Troubleshooting

**"No package.json found"**
Run DepPilot from inside a JavaScript/TypeScript project directory, or any subdirectory of one. It walks up the file tree to find the nearest `package.json`.

**"Multiple lock files detected"**
You have more than one lock file (e.g. both `yarn.lock` and `package-lock.json`). DepPilot will ask which one to use, or you can pass `--package-manager yarn|pnpm|npm` to skip the prompt.

**"Package manager not found in PATH"**
The detected package manager isn't installed or isn't in your `$PATH`. Install it, or override with `--package-manager`.

**Commit fails after update**
Your package manager may have modified files outside of `package.json` and the lock file (some tools do this). DepPilot will warn you. Use `--no-commit` to update without committing and handle git yourself.

**Validation keeps failing**
Run `deppilot update --check "your-command" --continue-on-error` to update everything and review failures in the summary at the end, instead of stopping on the first one.

---

## Contributing

Issues and pull requests are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for setup instructions and code style notes.

## Releasing

Maintainers: see [RELEASING.md](RELEASING.md) for the release process, required secrets, and recovery steps.

## License

MIT — see [LICENSE](LICENSE).
