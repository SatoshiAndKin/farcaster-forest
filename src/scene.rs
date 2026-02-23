use bevy::camera::ScalingMode;
use bevy::prelude::*;
use bevy_kira_audio::prelude::SpatialAudioReceiver;

use crate::GameState;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_scene);
    }
}

#[derive(Component)]
pub struct Tree;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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

        // Trunk
        commands.spawn((
            Mesh3d(trunk_mesh.clone()),
            MeshMaterial3d(trunk_material.clone()),
            Transform::from_translation(trunk_pos),
        ));

        // Canopy + Tree marker (birds target the canopy position)
        commands.spawn((
            Mesh3d(canopy_mesh.clone()),
            MeshMaterial3d(canopy_material.clone()),
            Transform::from_translation(canopy_pos),
            Tree,
        ));
    }

    // Directional light (sun)
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Ambient light
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.9, 0.9, 1.0),
        brightness: 300.0,
        ..default()
    });
}
