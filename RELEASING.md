# Releasing DepPilot

## Required secrets

Configure all four in **GitHub → Settings → Secrets and variables → Actions**:

| Secret | Where to get it | Required for |
|---|---|---|
| `GITHUB_TOKEN` | Provided automatically by GitHub Actions | GitHub Release |
| `CARGO_REGISTRY_TOKEN` | [crates.io/settings/tokens](https://crates.io/settings/tokens) → New Token (scope: publish) | crates.io |
| `NPM_TOKEN` | [npmjs.com/settings/~/tokens](https://www.npmjs.com/settings/~/tokens) → New Token → Publish | npm |
| `HOMEBREW_TAP_TOKEN` | GitHub → Developer settings → PATs → New token → select `repo` scope | Homebrew tap |

---

## Homebrew tap: one-time setup

Before your **first release**, create the tap repository:

```bash
# On GitHub: create a new public repo named "homebrew-deppilot"
# Then initialize it locally:

git clone https://github.com/mauuleo/homebrew-deppilot.git
cd homebrew-deppilot
mkdir -p Formula
touch Formula/.gitkeep
git add .
git commit -m "chore: initial tap"
git push
```

After that, every release automatically pushes the updated `Formula/deppilot.rb`.

---

## How to create a release

**1. Bump the version** across all files at once:

```bash
scripts/bump-version 1.2.0
```

This updates `Cargo.toml` and all `npm/*/package.json` files.

**2. Update CHANGELOG.md** — add an entry for the new version:

```markdown
## [1.2.0] - 2025-09-01

### Added
- ...

### Fixed
- ...
```

**3. Run pre-release checks** locally:

```bash
scripts/check-release
```

All checks must pass. Fix any failures before continuing.

**4. Commit, tag, and push**:

```bash
git add -A
git commit -m "chore: release v1.2.0"
git tag v1.2.0
git push origin main v1.2.0
```

**5. Watch the workflow**: [github.com/mauuleo/deppilot/actions](https://github.com/mauuleo/deppilot/actions)

The pipeline runs automatically:

```
validate
  └─► build (5 platforms in parallel)
         └─► checksum
                ├─► github-release
                ├─► publish-crates
                ├─► publish-npm
                └─► update-homebrew
```

---

## Testing a release locally

### Test the binary

```bash
cargo build --release
./target/release/deppilot --version
```

### Test cargo publish

```bash
cargo publish --dry-run
```

### Test the npm wrapper locally

```bash
# Build the binary first
cargo build --release

# Place it where the wrapper expects it
mkdir -p npm/deppilot-darwin-arm64/bin
cp target/release/deppilot npm/deppilot-darwin-arm64/bin/

# Run the wrapper (it resolves via the local path fallback)
node npm/deppilot/bin/deppilot.js --version
```

### Simulate the full npm packaging

```bash
cd npm/deppilot-darwin-arm64
npm pack   # inspect the .tgz contents
tar -tvf deppilot-darwin-arm64-*.tgz
```

---

## Recovery from partial failures

### crates.io publish failed

crates.io does **not** allow republishing the same version. Options:

- If the binary release was created: users can still `cargo install deppilot` (crates.io may accept a retry after a short delay — check the crates.io status page).
- If metadata is wrong: bump to a patch version (`scripts/bump-version 1.2.1`), fix the issue, and release again.

### npm publish partially failed (some platform packages published, root did not)

Manually publish what is missing. First download the release artifacts, then:

```bash
# Place binary in package dir
mkdir -p npm/deppilot-darwin-arm64/bin
curl -L https://github.com/mauuleo/deppilot/releases/download/v1.2.0/deppilot-macos-aarch64.tar.gz \
  | tar -xz -C npm/deppilot-darwin-arm64/bin/

cd npm/deppilot-darwin-arm64
npm publish --access public
```

To deprecate a broken platform package:

```bash
npm deprecate deppilot-darwin-arm64@1.2.0 "Broken release, use 1.2.1"
```

### Homebrew tap update failed

Manually update the formula:

```bash
git clone https://github.com/mauuleo/homebrew-deppilot.git
cd homebrew-deppilot

# Download checksums from the GitHub Release
curl -L https://github.com/mauuleo/deppilot/releases/download/v1.2.0/SHA256SUMS.txt

# Fill in the template manually
cp path/to/deppilot/homebrew/deppilot.rb.template Formula/deppilot.rb
# Edit VERSION and SHA256_* placeholders

git add Formula/deppilot.rb
git commit -m "Update deppilot to v1.2.0"
git push
```

### Rolling back a git tag

```bash
git push origin :v1.2.0   # delete remote tag
git tag -d v1.2.0          # delete local tag
```

> **Warning:** Deleting a tag does NOT undo crates.io or npm publishes.
> Neither registry allows unpublishing after the package is live.

---

## Version consistency rules

The following must always be in sync before a release:

| File | Field |
|---|---|
| `Cargo.toml` | `version` |
| `npm/deppilot/package.json` | `version` + all `optionalDependencies` |
| `npm/deppilot-darwin-arm64/package.json` | `version` |
| `npm/deppilot-darwin-x64/package.json` | `version` |
| `npm/deppilot-linux-x64/package.json` | `version` |
| `npm/deppilot-linux-arm64/package.json` | `version` |
| `npm/deppilot-win32-x64/package.json` | `version` |

`scripts/bump-version` updates all of them atomically.
`scripts/check-release` verifies all of them are consistent.
The `validate` job in the release workflow also checks them — and fails the entire release if they don't match.
