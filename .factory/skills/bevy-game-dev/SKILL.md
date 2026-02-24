---
name: Bevy Game Development
description: Expert guidance for developing games with the Bevy engine, including ECS patterns, asset management, and system organization
---

# Bevy Game Development Skill

This skill provides expertise in developing games using the Bevy engine (v0.18+).

## Core Concepts

### Entity Component System (ECS)
When working with Bevy:
- **Entities**: Unique identifiers for game objects (birds, trees, camera)
- **Components**: Data attached to entities (Transform, Mesh, Bird, Tree)
- **Systems**: Functions that operate on entities with specific components
- **Resources**: Global state shared across systems (DayClock, BirdSpawnTimer)

### System Organization
```rust
// Systems are functions that take World parameters
fn spawn_birds(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<BirdSpawnTimer>,
) {
    // System logic here
}
```

### Plugin Pattern
Organize related systems into plugins:
```rust
pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_birds);
    }
}
```

## Asset Management

### Loading Assets
Use `bevy_asset_loader` for efficient asset loading:
```rust
#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/bird.ogg")]
    pub bird_call: Handle<AudioSource>,
}
```

### Asset Paths
- Place assets in `assets/` directory
- Reference with relative paths from `assets/`
- Supported formats: OGG (audio), GLTF (3D models), PNG (textures)

## Common Patterns

### State Management
```rust
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Loading,
    Playing,
    Menu,
}

// Run systems conditionally
app.add_systems(Update, my_system.run_if(in_state(GameState::Playing)))
```

### Queries
```rust
// Query entities with specific components
fn update_birds(mut birds: Query<(&mut Transform, &Bird)>) {
    for (mut transform, bird) in &mut birds {
        // Update bird position
    }
}
```

### Commands for Spawning
```rust
commands.spawn((
    Transform::default(),
    Bird { species: BirdSpecies::MourningDove },
    // More components...
));
```

## Performance Tips

1. **Use Fixed Timestep** for physics and game logic
2. **Batch operations** in systems
3. **Avoid clone()** on handles - use `.clone()` method
4. **Use change detection** with `Changed<T>` filter
5. **Parallelize systems** - Bevy does this automatically when possible

## Platform-Specific Considerations

### Web (WASM)
- Use `wasm32-unknown-unknown` target
- Build with `trunk serve` for development
- Audio may have browser-specific issues
- File size matters - optimize assets

### Mobile
- iOS: Requires Xcode and simulator setup
- Android: Requires Android SDK and NDK
- Use orthographic projection for better performance
- Test on actual devices for audio/input

### Desktop
- Native builds are straightforward
- Use `cargo run` for development
- Platform-specific features via feature flags

## Debugging

### Common Issues
1. **"Entity not found"**: Entity was despawned but reference kept
2. **System ordering**: Use `.before()` and `.after()` to order systems
3. **Mutable access conflicts**: Two systems can't mutably access same resource
4. **Missing components**: Query won't match entities without all components

### Diagnostics
```rust
app.add_plugins(FrameTimeDiagnosticsPlugin::default());
app.add_plugins(LogDiagnosticsPlugin::default());
```

## Testing Bevy Code

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bird_behavior() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        // Test logic
    }
}
```

### Integration Tests
- Use `MinimalPlugins` instead of `DefaultPlugins` for headless testing
- Mock time with `Time::update()`
- Test systems in isolation

## Resources

- [Bevy Book](https://bevyengine.org/learn/book/)
- [Bevy Cheat Book](https://bevy-cheatbook.github.io/)
- [Bevy Examples](https://github.com/bevyengine/bevy/tree/main/examples)
- [Bevy Discord](https://discord.gg/bevy)

## When to Use This Skill

Use this skill when:
- Creating new game systems or components
- Organizing code into plugins
- Debugging ECS-related issues
- Optimizing game performance
- Implementing platform-specific features
- Setting up asset pipelines
