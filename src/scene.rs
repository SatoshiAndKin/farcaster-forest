use bevy::camera::ScalingMode;
use bevy::prelude::*;
use bevy_kira_audio::prelude::SpatialAudioReceiver;

use crate::GameState;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_scene)
            .add_systems(
                Update,
                update_day_night_cycle.run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct Tree;

#[derive(Component)]
struct Sun;

/// Full day cycle duration in seconds. 120s = 2 minute day.
const DAY_DURATION: f32 = 120.0;

#[derive(Resource)]
struct DayClock {
    elapsed: f32,
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(DayClock { elapsed: 0.0 });

    // Orthographic isometric camera (visual only)
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 18.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(20.0, 20.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Spatial audio receiver at ground level in the clearing
    let listener_pos = Vec3::new(0.0, 1.5, 0.0);
    commands.spawn((
        Transform::from_translation(listener_pos),
        SpatialAudioReceiver,
    ));

    // Blue sphere showing where the listener is
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.4).mesh().uv(16, 12))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.3, 0.9),
            ..default()
        })),
        Transform::from_translation(listener_pos),
    ));

    // Ground plane
    let ground_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.35, 0.55, 0.25),
        perceptual_roughness: 0.9,
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(ground_material),
    ));

    // Tree materials
    let trunk_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.45, 0.3, 0.15),
        perceptual_roughness: 0.9,
        ..default()
    });
    let canopy_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.5, 0.15),
        perceptual_roughness: 0.8,
        ..default()
    });
    let trunk_mesh = meshes.add(Cylinder::new(0.2, 2.0));
    let canopy_mesh = meshes.add(Sphere::new(1.2).mesh().uv(16, 12));

    let tree_positions = [
        Vec3::new(-8.0, 0.0, -6.0),
        Vec3::new(-5.0, 0.0, 3.0),
        Vec3::new(-3.0, 0.0, -9.0),
        Vec3::new(0.0, 0.0, 7.0),
        Vec3::new(2.0, 0.0, -4.0),
        Vec3::new(5.0, 0.0, 1.0),
        Vec3::new(7.0, 0.0, -7.0),
        Vec3::new(8.0, 0.0, 5.0),
        Vec3::new(-6.0, 0.0, 8.0),
        Vec3::new(4.0, 0.0, -1.0),
    ];

    for pos in &tree_positions {
        let trunk_pos = *pos + Vec3::new(0.0, 1.0, 0.0);
        let canopy_pos = *pos + Vec3::new(0.0, 2.8, 0.0);

        commands.spawn((
            Mesh3d(trunk_mesh.clone()),
            MeshMaterial3d(trunk_material.clone()),
            Transform::from_translation(trunk_pos),
        ));

        commands.spawn((
            Mesh3d(canopy_mesh.clone()),
            MeshMaterial3d(canopy_material.clone()),
            Transform::from_translation(canopy_pos),
            Tree,
        ));
    }

    // Sun directional light -- initial position set by update_day_night_cycle
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        Sun,
    ));

    // Start with a neutral ambient; the cycle system will adjust it each frame
    commands.insert_resource(GlobalAmbientLight {
        color: Color::WHITE,
        brightness: 200.0,
        ..default()
    });
}

/// Map a 0..1 day progress to sun angle, color, illuminance, and ambient values.
fn update_day_night_cycle(
    time: Res<Time>,
    mut clock: ResMut<DayClock>,
    mut sun_query: Query<(&mut DirectionalLight, &mut Transform), With<Sun>>,
    mut ambient: ResMut<GlobalAmbientLight>,
) {
    clock.elapsed += time.delta_secs();
    let t = (clock.elapsed % DAY_DURATION) / DAY_DURATION; // 0..1

    // Sun angle: t=0 sunrise (east horizon), t=0.25 noon (top), t=0.5 sunset (west horizon),
    // t=0.5..1.0 nighttime (sun below horizon)
    let sun_angle = t * std::f32::consts::TAU; // full circle
    let sun_y = sun_angle.sin();
    let sun_x = sun_angle.cos();
    // Sun orbits in the XY plane, offset on Z so it arcs overhead
    let sun_dir = Vec3::new(sun_x, sun_y, 0.3).normalize();

    let is_day = sun_y > -0.05; // slight grace below horizon for twilight

    // Color temperature based on sun elevation
    let (sun_color, sun_lux, ambient_color, ambient_brightness) = if sun_y > 0.3 {
        // High sun: bright white-yellow daylight
        let warmth = 1.0 - (sun_y - 0.3) / 0.7; // 0 at zenith, 1 near horizon
        let r = 1.0;
        let g = 0.95 - warmth * 0.1;
        let b = 0.85 - warmth * 0.2;
        (
            Color::srgb(r, g, b),
            10000.0 * sun_y.min(1.0),
            Color::srgb(0.85, 0.9, 1.0),
            250.0,
        )
    } else if sun_y > 0.0 {
        // Low sun: warm sunrise/sunset orange
        let frac = sun_y / 0.3;
        let r = 1.0;
        let g = 0.5 + frac * 0.4;
        let b = 0.2 + frac * 0.4;
        (
            Color::srgb(r, g, b),
            2000.0 + frac * 8000.0,
            Color::srgb(0.8, 0.6, 0.4),
            100.0 + frac * 150.0,
        )
    } else if sun_y > -0.05 {
        // Twilight: deep orange fading to blue
        let frac = (sun_y + 0.05) / 0.05; // 1 at horizon, 0 at -0.05
        let r = 0.8 * frac;
        let g = 0.3 * frac;
        let b = 0.1 * frac + 0.15 * (1.0 - frac);
        (
            Color::srgb(r, g, b),
            500.0 * frac,
            Color::srgb(0.15 + 0.5 * frac, 0.15 + 0.3 * frac, 0.3 + 0.2 * frac),
            30.0 + 70.0 * frac,
        )
    } else {
        // Night: moonlight (cool blue, very dim)
        (
            Color::srgb(0.4, 0.45, 0.6),
            200.0,
            Color::srgb(0.08, 0.08, 0.15),
            20.0,
        )
    };

    if let Ok((mut light, mut transform)) = sun_query.single_mut() {
        light.color = sun_color;
        light.illuminance = sun_lux;

        if is_day {
            // Sun shines down from its orbital position
            let sun_pos = sun_dir * 50.0;
            *transform = Transform::from_translation(sun_pos).looking_at(Vec3::ZERO, Vec3::Y);
        } else {
            // Night: position the light as moonlight from opposite side, high up
            let moon_dir = Vec3::new(-sun_x, 0.5, -0.3).normalize();
            let moon_pos = moon_dir * 50.0;
            *transform = Transform::from_translation(moon_pos).looking_at(Vec3::ZERO, Vec3::Y);
        }
    }

    ambient.color = ambient_color;
    ambient.brightness = ambient_brightness;
}
