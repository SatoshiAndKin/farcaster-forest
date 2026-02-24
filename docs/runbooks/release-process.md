# Runbook: Release Process

## Problem Description
Create and publish a new version release of Farcaster Forest.

## Prerequisites
- Write access to the repository
- All CI checks passing on main branch
- Release notes prepared (or use automated generation)

## Steps

### 1. Prepare Release
1. Ensure main branch is clean and up to date:
   ```bash
   git checkout main
   git pull origin main
   ```

2. Update version in `Cargo.toml` if needed:
   ```toml
   [package]
   version = "0.2.0"  # Increment as needed
   ```

3. Update `Cargo.lock`:
   ```bash
   cargo build
   ```

4. Commit version bump (if changed):
   ```bash
   git add Cargo.toml Cargo.lock
   git commit -m "chore(release): bump version to 0.2.0"
   git push origin main
   ```

### 2. Create Git Tag
1. Create annotated tag following semver (v0.2.0):
   ```bash
   git tag -a v0.2.0 -m "Release v0.2.0"
   ```

2. Push tag to trigger release workflow:
   ```bash
   git push origin v0.2.0
   ```

### 3. Monitor Release Workflow
1. Go to GitHub Actions: https://github.com/SatoshiAndKin/farcaster-forest/actions
2. Watch the `release-flow` workflow
3. Workflow builds for:
   - Windows (ZIP)
   - Linux (tar.gz)
   - macOS (DMG)
   - Web (WASM ZIP)
   - iOS (unsigned IPA)
   - Android (APK)

### 4. Verify Release Assets
1. Go to Releases page: https://github.com/SatoshiAndKin/farcaster-forest/releases
2. Verify release was created with tag `v0.2.0`
3. Confirm all platform builds are attached
4. Test download one asset to verify

### 5. Update Release Notes (Optional)
1. Edit the release on GitHub
2. Add generated changelog from `git-cliff`:
   ```bash
   git cliff --tag v0.2.0 --strip header
   ```
3. Or manually write release notes highlighting key changes

## Verification
- Release appears on GitHub Releases page
- All platform builds are available as downloadable assets
- Release notes are clear and accurate
- Users can download and run the game

## Prevention
- Automate version bumping with tools like `cargo-release`
- Use conventional commits for better changelog generation
- Consider branch protection requiring PR reviews before releases

## Troubleshooting

### Release workflow fails
- Check GitHub Actions logs for specific error
- Common issues: signing certificates (mobile), missing secrets, build failures
- Can re-run failed jobs from Actions UI

### Tag already exists
- Delete tag locally and remotely:
  ```bash
  git tag -d v0.2.0
  git push origin :refs/tags/v0.2.0
  ```
- Fix issues and recreate tag

## Related Resources
- [Release Workflow](../../.github/workflows/release.yaml)
- [Release Notes Workflow](../../.github/workflows/release-notes.yml)
- [git-cliff Config](../../cliff.toml)
