# AGENTS.md

Essential information for AI agents working with Farcaster Forest.

## Project Overview

Farcaster Forest is a Bevy game featuring a peaceful forest with song birds. It's a multi-platform application targeting desktop (Windows, Linux, macOS), web (WASM), and mobile (iOS, Android).

## Quick Start

### Native Development
```bash
cargo run
```

### Web Development
```bash
# First time setup
cargo install --locked trunk
rustup target add wasm32-unknown-unknown

# Run development server
trunk serve
# App available at http://localhost:8080
```

### Mobile Development

**Android:**
```bash
cargo apk run -p mobile
```

**iOS:**
```bash
cd mobile
make run
```

## Build Commands

### Development Build
```bash
cargo build
```

### Release Build
```bash
cargo build --profile dist
```

### Web Build
```bash
trunk build --release
```

## Testing

### Run All Tests
```bash
cargo test
```

### Run Tests with Output
```bash
cargo test -- --nocapture
```

### Run Doc Tests
```bash
cargo test --doc --all-features
```

### Check for Test Collection (Fast)
```bash
cargo test -- --list
```

## Code Quality

### Linting
```bash
# Run clippy
cargo clippy --workspace --all-targets --all-features

# Clippy with warnings as errors (same as CI)
cargo clippy --workspace --all-targets --all-features -- -Dwarnings
```

### Formatting
```bash
# Check format
cargo fmt --all -- --check

# Auto-format
cargo fmt --all
```

## CI/CD

- **CI Checks**: Every push runs tests, clippy, and format checks on Windows, Linux, and macOS
- **Release**: Push a git tag in the format `v*.*.*` (e.g., `v1.0.0`) to trigger multi-platform release builds
- **Deployment**: Run the `deploy-github-page` workflow to deploy web build to GitHub Pages

## Project Structure

```
farcaster-forest/
├── src/              # Main game source code
│   ├── lib.rs       # Library entry point
│   ├── main.rs      # Binary entry point
│   ├── bird.rs      # Bird behavior and species definitions
│   ├── scene.rs     # 3D scene and environment
│   ├── audio.rs     # Audio system
│   ├── loading.rs   # Asset loading
│   └── menu.rs      # UI and menu
├── mobile/          # Mobile-specific build configuration
├── assets/          # Game assets (audio, models, textures)
├── build/           # Platform-specific build resources
└── examples/        # Example code
```

## Key Technologies

- **Bevy 0.18**: Game engine
- **bevy_kira_audio**: Spatial audio
- **bevy_asset_loader**: Asset management
- **Trunk**: WASM build tool
- **Rust nightly**: Uses edition 2024 and nightly toolchain

## Development Environment

### Nix Support
```bash
# On non-NixOS Linux systems
nix develop --impure

# Then use cargo commands as usual (prefix with 'gl' if using nixgl)
gl cargo run
```

### Required Rust Toolchain
```bash
rustup toolchain install nightly-2026-02-23
rustup component add rustfmt clippy rust-analyzer
rustup target add wasm32-unknown-unknown
```

### Platform-Specific Dependencies

**Linux:**
```bash
sudo apt-get update
sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 libwayland-dev
```

**macOS:**
No additional dependencies required.

**Windows:**
No additional dependencies required.

## Common Tasks

### Add a New Bird Species
1. Add audio files to `assets/audio/`
2. Update bird species in `src/bird.rs`
3. Add credits to `credits/CREDITS.md`

### Modify Day/Night Cycle
Edit the lighting and sun movement code in `src/scene.rs`

### Change Audio Behavior
Spatial audio configuration is in `src/audio.rs`

## Conventions

### Code Style
- Use `cargo fmt` for consistent formatting
- Follow Rust API guidelines: https://rust-lang.github.io/api-guidelines/
- Clippy lints are enforced in CI

### Naming Conventions
Rust naming conventions are enforced through clippy and documented here for reference:

- **Types, Traits, Enums**: `PascalCase`
  - Examples: `BirdSpecies`, `GameState`, `DayClock`, `AudioAssets`
- **Functions, Methods, Variables**: `snake_case`
  - Examples: `spawn_birds()`, `update_day_night_cycle()`, `bird_count`
- **Constants, Statics**: `SCREAMING_SNAKE_CASE`
  - Examples: `MAX_BIRDS`, `DAY_DURATION`, `NOON_SHADOW_STRENGTH`
- **Enum Variants**: `PascalCase`
  - Examples: `GameState::Playing`, `BirdSpecies::MourningDove`
- **Modules**: `snake_case`
  - Examples: `bird`, `scene`, `audio`, `loading`

These conventions are enforced by clippy in CI and configured in `clippy.toml`.

### Git Workflow
- Make atomic commits with clear messages
- Reference issue numbers in commits when applicable
- CI must pass before merging

### Asset Management
- Keep `credits/CREDITS.md` up to date for all third-party assets
- Use OGG format for audio files (not MP3)
- Compress assets appropriately for web deployment

### Logging and Security
- Use Rust's `log` crate for structured logging
- Log levels: ERROR for failures, WARN for issues, INFO for key events, DEBUG for development
- **Never log sensitive data**: passwords, tokens, API keys, or personal information
- When logging user input or external data, sanitize before logging
- Use `log::max_level_*` features to control log verbosity in release builds
- Example sanitization:
  ```rust
  // BAD: log!("User {} logged in with password {}", username, password);
  // GOOD: log!("User {} logged in", username.chars().take(3).collect::<String>() + "***");
  ```

## Troubleshooting

### Web Audio Issues
Some browsers may have performance issues with audio. This is a known issue tracked at https://github.com/NiklasEi/bevy_kira_audio/issues/9

### Build Failures
- Ensure you're using the correct nightly toolchain: `nightly-2026-02-23`
- Clean build artifacts: `cargo clean`
- Update dependencies: `cargo update`

## Resources

- Bevy Documentation: https://bevyengine.org/learn/
- Bevy Cheat Book: https://bevy-cheatbook.github.io/
- Bevy Discord: https://discord.gg/bevy
- Original Template: https://github.com/NiklasEi/bevy_game_template
