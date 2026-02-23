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
const NOON_SHADOW_STRENGTH: f32 = 0.7;

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
        DistanceFog {
            color: Color::srgba(0.75, 0.83, 0.94, 1.0),
            directional_light_exponent: 18.0,
            directional_light_color: Color::srgba(1.0, 0.97, 0.92, 1.0),
            falloff: FogFalloff::Linear {
                start: 28.0,
                end: 100.0,
            }
        },
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
        // High sun: near-neutral daylight, warmer as it approaches lower elevation
        let warmth = 1.0 - (sun_y - 0.3) / 0.7; // 0 at zenith, 1 near horizon
        let r = 1.0;
        let g = 0.97 - warmth * 0.05;
        let b = 0.93 - warmth * 0.1;
        (
            Color::srgb(r, g, b),
            40000.0 * sun_y.min(1.0),
            Color::srgb(0.56, 0.64, 0.78),
            85.0,
        )
    } else if sun_y > 0.0 {
        // Low sun: warm golden hour
        let frac = sun_y / 0.3;
        let r = 1.0;
        let g = 0.62 + frac * 0.3;
        let b = 0.38 + frac * 0.36;
        (
            Color::srgb(r, g, b),
            1800.0 + frac * 10200.0,
            Color::srgb(0.42 + frac * 0.12, 0.4 + frac * 0.18, 0.5 + frac * 0.2),
            28.0 + frac * 58.0,
        )
    } else if sun_y > -0.05 {
        // Twilight: rapidly cooling sky and very low direct light
        let frac = (sun_y + 0.05) / 0.05; // 1 at horizon, 0 at -0.05
        let r = 0.95 * frac;
        let g = 0.5 * frac;
        let b = 0.25 * frac + 0.3 * (1.0 - frac);
        (
            Color::srgb(r, g, b),
            400.0 * frac,
            Color::srgb(0.15 + 0.35 * frac, 0.17 + 0.28 * frac, 0.3 + 0.22 * frac),
            14.0 + 30.0 * frac,
        )
    } else {
        // Night: cool moonlight with low sky fill
        (
            Color::srgb(0.52, 0.56, 0.67),
            120.0,
            Color::srgb(0.07, 0.08, 0.13),
            12.0,
        )
    };

    if let Ok((mut light, mut transform)) = sun_query.single_mut() {
        light.color = sun_color;
        light.illuminance = sun_lux;
        let noon = sun_y.clamp(0.0, 1.0);
        light.shadow_depth_bias = 0.02 - (0.008 * noon * NOON_SHADOW_STRENGTH);
        light.shadow_normal_bias = 2.0 - (0.8 * noon * NOON_SHADOW_STRENGTH);

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
