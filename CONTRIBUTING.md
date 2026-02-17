# Contributing to PNG-DB

## Development Setup

1. Install Rust: https://rustup.rs/
2. Install wasm-pack: `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`
3. Clone the repository
4. Build: `just build` or `just build-wasm`

## Building

### CLI Version
```bash
just build
```

### WASM Version
```bash
just build-wasm
# Or directly:
./build-wasm.sh
```

The WASM build creates a `pkg/` directory with the npm package ready to publish.

## Release Process

This project uses [release-plz](https://release-plz.ieni.dev/) for automated releases.

### How It Works

1. **Make changes** and commit using [Conventional Commits](https://www.conventionalcommits.org/):
   - `feat:` for new features (minor version bump)
   - `fix:` for bug fixes (patch version bump)
   - `feat!:` or `fix!:` for breaking changes (major version bump)
   - `docs:`, `chore:`, `refactor:`, etc. for other changes

2. **Push to main** - release-plz will automatically:
   - Create/update a release PR with changelog
   - Update `Cargo.toml` version
   - Update `CHANGELOG.md`

3. **Merge the release PR** - release-plz will:
   - Create a GitHub release
   - Tag the release

4. **npm publish** happens automatically via GitHub Actions when a new release is published.

### Manual Release (if needed)

If you need to manually trigger the npm publish:
1. Go to Actions → "Publish to npm" → Run workflow

## Commit Message Examples

```bash
# New feature (minor version bump)
git commit -m "feat: add support for nested JSON queries"

# Bug fix (patch version bump)
git commit -m "fix: correct coordinate overflow in large images"

# Breaking change (major version bump)
git commit -m "feat!: change query syntax to support OR conditions"

# Documentation
git commit -m "docs: update installation instructions"

# Chore
git commit -m "chore: update dependencies"
```

## Testing

```bash
# Run Rust tests
just test

# Try the demo
just build
just demo
```

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` to check for common mistakes
- Follow Rust naming conventions
