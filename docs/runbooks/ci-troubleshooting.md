# Runbook: CI/CD Troubleshooting

## Problem Description
GitHub Actions workflows fail, causing CI checks to block PRs or deployments.

## Prerequisites
- Access to GitHub repository
- Understanding of which workflow failed (check GitHub Actions tab)

## Common Scenarios

### Scenario 1: Test Failures

**Symptoms:**
- CI job "test" fails
- Red X on PR/commit

**Steps:**
1. Click on failed workflow in GitHub Actions
2. Expand the "Build & run tests" step
3. Identify which test failed
4. Run test locally to reproduce:
   ```bash
   cargo test
   # Or specific test:
   cargo test test_name
   ```
5. Fix the failing test
6. Commit and push fix

**Verification:**
CI test job passes and shows green checkmark.

### Scenario 2: Clippy Warnings

**Symptoms:**
- "lint" job fails
- Error: "warnings treated as errors"

**Steps:**
1. Run clippy locally with same flags as CI:
   ```bash
   cargo clippy --workspace --all-targets --all-features -- -Dwarnings
   ```
2. Fix reported warnings
3. Verify locally:
   ```bash
   cargo clippy --workspace --all-targets --all-features -- -Dwarnings
   ```
4. Commit and push fixes

**Verification:**
Clippy job passes without warnings.

### Scenario 3: Format Check Failures

**Symptoms:**
- "lint" job fails on format check
- "Diff in /path/to/file.rs"

**Steps:**
1. Format code locally:
   ```bash
   cargo fmt --all
   ```
2. Review changes:
   ```bash
   git diff
   ```
3. Commit formatting:
   ```bash
   git add .
   git commit -m "style: apply cargo fmt"
   git push
   ```

**Verification:**
Format check passes in CI.

### Scenario 4: Dependency Cache Issues

**Symptoms:**
- Slow CI or cache-related errors
- "failed to restore cache"

**Steps:**
1. Manually clear cache on GitHub:
   - Settings > Actions > Caches
   - Delete old caches for affected branch
2. Re-run workflow

**Verification:**
Cache is rebuilt successfully and subsequent runs are faster.

### Scenario 5: Platform-Specific Failures

**Symptoms:**
- Tests pass on some platforms but fail on others (Windows/Linux/macOS)

**Steps:**
1. Identify which platform failed
2. Check for platform-specific code or assumptions
3. Test locally on that platform if available
4. Use conditional compilation if needed:
   ```rust
   #[cfg(target_os = "windows")]
   fn platform_specific() { ... }
   ```
5. Fix and test

**Verification:**
All platform jobs pass in CI.

### Scenario 6: Workflow Syntax Errors

**Symptoms:**
- Workflow doesn't run at all
- Error: "Invalid workflow file"

**Steps:**
1. Validate YAML syntax:
   ```bash
   # Use online YAML validator or:
   python -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"
   ```
2. Check GitHub Actions documentation for correct syntax
3. Fix syntax errors
4. Commit and push

**Verification:**
Workflow appears in Actions tab and runs.

## Prevention

1. **Run locally before pushing:**
   ```bash
   cargo test
   cargo clippy --workspace --all-targets --all-features -- -Dwarnings
   cargo fmt --all -- --check
   ```

2. **Use pre-commit hooks:**
   ```bash
   pre-commit install
   ```

3. **Test on multiple platforms** before creating PR (if changes are platform-specific)

4. **Keep dependencies updated** to avoid version conflicts

## Quick Commands Reference

```bash
# Run all checks locally (same as CI)
cargo test
cargo clippy --workspace --all-targets --all-features -- -Dwarnings
cargo fmt --all -- --check

# Fix formatting
cargo fmt --all

# Clean and rebuild
cargo clean && cargo build

# Update dependencies
cargo update
```

## Related Resources
- [CI Workflow](../../.github/workflows/ci.yml)
- [Pre-commit Config](../../.pre-commit-config.yaml)
- [AGENTS.md](../../AGENTS.md) - Development commands
