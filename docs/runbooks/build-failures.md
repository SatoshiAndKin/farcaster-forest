# Runbook: Build Failures

## Problem Description
Build fails locally or in CI with compilation errors or dependency issues.

## Prerequisites
- Git repository cloned
- Rust toolchain installed

## Common Scenarios

### Scenario 1: Wrong Toolchain Version

**Symptoms:**
- Error: "requires nightly features" or version mismatch errors

**Steps:**
1. Check required toolchain version in `rust-toolchain.toml`
2. Install correct toolchain:
   ```bash
   rustup toolchain install nightly-2026-02-23
   rustup component add rustfmt clippy rust-analyzer --toolchain nightly-2026-02-23
   ```
3. Verify installation:
   ```bash
   rustup show
   ```

**Verification:**
```bash
cargo build
```

### Scenario 2: Dependency Resolution Failures

**Symptoms:**
- Error: "failed to select a version" or dependency conflicts

**Steps:**
1. Clean build artifacts:
   ```bash
   cargo clean
   ```
2. Update Cargo.lock:
   ```bash
   cargo update
   ```
3. Rebuild:
   ```bash
   cargo build
   ```

**Verification:**
Build completes successfully without errors.

### Scenario 3: Platform-Specific Dependencies (Linux)

**Symptoms:**
- Error: "linker `cc` not found" or missing system libraries

**Steps:**
1. Install required system dependencies:
   ```bash
   sudo apt-get update
   sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 libwayland-dev
   ```
2. Retry build:
   ```bash
   cargo build
   ```

**Verification:**
Build completes without linker errors.

### Scenario 4: Web Build Failures (WASM)

**Symptoms:**
- Trunk build fails or target not found

**Steps:**
1. Add WASM target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
2. Install trunk if needed:
   ```bash
   cargo install --locked trunk
   ```
3. Build for web:
   ```bash
   trunk build --release
   ```

**Verification:**
Web build completes and creates `dist/` directory with index.html.

## Prevention

1. **Use rust-toolchain.toml** - Already configured in repo
2. **Commit Cargo.lock** - Already done, ensures reproducible builds
3. **Document dependencies** - System dependencies listed in AGENTS.md
4. **CI testing** - Multiple platforms tested in GitHub Actions

## Related Resources
- [AGENTS.md](../../AGENTS.md) - Build commands and setup
- [CI Workflow](../../.github/workflows/ci.yml) - Automated build configuration
