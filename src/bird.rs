use bevy::prelude::*;
use bevy_kira_audio::SpatialRadius;
use bevy_kira_audio::prelude::*;
use rand::Rng;

use crate::GameState;
use crate::loading::AudioAssets;
use crate::scene::Tree;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DidFixedTimestepRunThisFrame>()
            .init_resource::<BirdSpawnTimer>()
            .add_systems(FixedPreUpdate, set_fixed_timestep_flag)
            .add_systems(PreUpdate, clear_fixed_timestep_flag)
            .add_systems(
                FixedUpdate,
                advance_bird_physics.run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                RunFixedMainLoop,
                interpolate_bird_transforms
                    .in_set(RunFixedMainLoopSystems::AfterFixedMainLoop)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (spawn_birds, bird_ai, despawn_distant_birds).run_if(in_state(GameState::Playing)),
            );
    }
}

// -- Fixed timestep flag --

#[derive(Resource, Default, Deref, DerefMut)]
struct DidFixedTimestepRunThisFrame(bool);

fn clear_fixed_timestep_flag(mut flag: ResMut<DidFixedTimestepRunThisFrame>) {
    flag.0 = false;
}

fn set_fixed_timestep_flag(mut flag: ResMut<DidFixedTimestepRunThisFrame>) {
    flag.0 = true;
}

// -- Species --

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BirdSpecies {
    MourningDove,
    DownyWoodpecker,
    NorthernFlicker,
    StellersJay,
    CaliforniaScrubJay,
    BlackCappedChickadee,
    WhiteBreastedNuthatch,
    WhiteCrownedSparrow,
    RedWingedBlackbird,
    CassinsFinch,
    HouseFinch,
    PineSiskin,
    AmericanGoldfinch,
    EveningGrosbeak,
}

impl BirdSpecies {
    const ALL: &[BirdSpecies] = &[
        BirdSpecies::MourningDove,
        BirdSpecies::DownyWoodpecker,
        BirdSpecies::NorthernFlicker,
        BirdSpecies::StellersJay,
        BirdSpecies::CaliforniaScrubJay,
        BirdSpecies::BlackCappedChickadee,
        BirdSpecies::WhiteBreastedNuthatch,
        BirdSpecies::WhiteCrownedSparrow,
        BirdSpecies::RedWingedBlackbird,
        BirdSpecies::CassinsFinch,
        BirdSpecies::HouseFinch,
        BirdSpecies::PineSiskin,
        BirdSpecies::AmericanGoldfinch,
        BirdSpecies::EveningGrosbeak,
    ];

    fn color(&self) -> Color {
        match self {
            Self::MourningDove => Color::srgb(0.6, 0.5, 0.4),
            Self::DownyWoodpecker => Color::srgb(0.2, 0.2, 0.2),
            Self::NorthernFlicker => Color::srgb(0.7, 0.5, 0.3),
            Self::StellersJay => Color::srgb(0.1, 0.2, 0.6),
            Self::CaliforniaScrubJay => Color::srgb(0.3, 0.4, 0.7),
            Self::BlackCappedChickadee => Color::srgb(0.8, 0.8, 0.7),
            Self::WhiteBreastedNuthatch => Color::srgb(0.5, 0.5, 0.6),
            Self::WhiteCrownedSparrow => Color::srgb(0.6, 0.55, 0.45),
            Self::RedWingedBlackbird => Color::srgb(0.1, 0.1, 0.1),
            Self::CassinsFinch => Color::srgb(0.7, 0.3, 0.3),
            Self::HouseFinch => Color::srgb(0.8, 0.3, 0.2),
            Self::PineSiskin => Color::srgb(0.6, 0.6, 0.3),
            Self::AmericanGoldfinch => Color::srgb(0.9, 0.8, 0.1),
            Self::EveningGrosbeak => Color::srgb(0.7, 0.6, 0.1),
        }
    }

    fn radius(&self) -> f32 {
        match self {
            Self::MourningDove => 0.25,
            Self::DownyWoodpecker => 0.18,
            Self::NorthernFlicker => 0.22,
            Self::StellersJay => 0.24,
            Self::CaliforniaScrubJay => 0.22,
            Self::BlackCappedChickadee => 0.15,
            Self::WhiteBreastedNuthatch => 0.16,
            Self::WhiteCrownedSparrow => 0.17,
            Self::RedWingedBlackbird => 0.20,
            Self::CassinsFinch => 0.16,
            Self::HouseFinch => 0.16,
            Self::PineSiskin => 0.14,
            Self::AmericanGoldfinch => 0.14,
            Self::EveningGrosbeak => 0.20,
        }
    }

    /// Flight speed in units/second
    fn speed(&self) -> f32 {
        match self {
            Self::MourningDove => 1.5,
            Self::DownyWoodpecker => 1.0,
            Self::NorthernFlicker => 1.1,
            Self::StellersJay => 1.3,
            Self::CaliforniaScrubJay => 1.2,
            Self::BlackCappedChickadee => 0.9,
            Self::WhiteBreastedNuthatch => 0.8,
            Self::WhiteCrownedSparrow => 1.0,
            Self::RedWingedBlackbird => 1.4,
            Self::CassinsFinch => 1.1,
            Self::HouseFinch => 1.0,
            Self::PineSiskin => 0.9,
            Self::AmericanGoldfinch => 1.0,
            Self::EveningGrosbeak => 1.2,
        }
    }

    fn call_handles(
        &self,
        audio_assets: &AudioAssets,
    ) -> Vec<Handle<bevy_kira_audio::AudioSource>> {
        match self {
            Self::MourningDove => vec![audio_assets.mourning_dove_song.clone()],
            Self::DownyWoodpecker => vec![
                audio_assets.downy_woodpecker_calls.clone(),
                audio_assets.downy_woodpecker_drum.clone(),
            ],
            Self::NorthernFlicker => vec![
                audio_assets.northern_flicker_call.clone(),
                audio_assets.northern_flicker_call_2.clone(),
                audio_assets.northern_flicker_drum.clone(),
            ],
            Self::StellersJay => vec![
                audio_assets.stellers_jay_call.clone(),
                audio_assets.stellers_jay_calls.clone(),
            ],
            Self::CaliforniaScrubJay => vec![audio_assets.california_scrub_jay_calls.clone()],
            Self::BlackCappedChickadee => vec![
                audio_assets.black_capped_chickadee_song.clone(),
                audio_assets.black_capped_chickadee_call.clone(),
            ],
            Self::WhiteBreastedNuthatch => vec![
                audio_assets.white_breasted_nuthatch_song.clone(),
                audio_assets.white_breasted_nuthatch_call_1.clone(),
                audio_assets.white_breasted_nuthatch_call_2.clone(),
            ],
            Self::WhiteCrownedSparrow => vec![
                audio_assets.white_crowned_sparrow_song_1.clone(),
                audio_assets.white_crowned_sparrow_song_2.clone(),
                audio_assets.white_crowned_sparrow_call.clone(),
            ],
            Self::RedWingedBlackbird => vec![
                audio_assets.red_winged_blackbird_song.clone(),
                audio_assets.red_winged_blackbird_calls.clone(),
            ],
            Self::CassinsFinch => vec![
                audio_assets.cassins_finch_song.clone(),
                audio_assets.cassins_finch_call.clone(),
            ],
            Self::HouseFinch => vec![
                audio_assets.house_finch_song.clone(),
                audio_assets.house_finch_call.clone(),
            ],
            Self::PineSiskin => vec![audio_assets.pine_siskin_song_calls.clone()],
            Self::AmericanGoldfinch => vec![audio_assets.american_goldfinch_song_call.clone()],
            Self::EveningGrosbeak => vec![audio_assets.evening_grosbeak_calls.clone()],
        }
    }
}

// -- Bird components --

#[derive(Component)]
pub struct Bird {
    species: BirdSpecies,
    trees_visited: u32,
    max_trees: u32,
}

#[derive(Component)]
enum BirdState {
    Approaching { target: Vec3 },
    Perching { timer: Timer },
    Vocalizing { timer: Timer },
    FlyingToNext { target: Vec3 },
    Departing { target: Vec3 },
}

#[derive(Component, Default, Deref, DerefMut)]
struct PhysicalTranslation(Vec3);

#[derive(Component, Default, Deref, DerefMut)]
struct PreviousPhysicalTranslation(Vec3);

#[derive(Component, Default, Deref, DerefMut)]
struct Velocity(Vec3);

#[derive(Component)]
struct BirdCallHandles(Vec<Handle<bevy_kira_audio::AudioSource>>);

#[derive(Component)]
struct ActiveCall(#[allow(dead_code)] Handle<AudioInstance>);

// -- Spawn timer --

#[derive(Resource)]
struct BirdSpawnTimer {
    timer: Timer,
}

impl Default for BirdSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(3.0, TimerMode::Once),
        }
    }
}

const MAX_BIRDS: usize = 5;

#[allow(clippy::too_many_arguments)]
fn spawn_birds(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<BirdSpawnTimer>,
    audio_assets: Res<AudioAssets>,
    birds: Query<&Bird>,
    trees: Query<&Transform, With<Tree>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.is_finished() {
        return;
    }

    let bird_count = birds.iter().count();
    if bird_count >= MAX_BIRDS {
        // Reset timer and wait
        let mut rng = rand::rng();
        spawn_timer.timer = Timer::from_seconds(rng.random_range(5.0..10.0), TimerMode::Once);
        return;
    }

    let tree_positions: Vec<Vec3> = trees.iter().map(|t| t.translation).collect();
    if tree_positions.is_empty() {
        return;
    }

    let mut rng = rand::rng();
    let species = BirdSpecies::ALL[rng.random_range(0..BirdSpecies::ALL.len())];
    let call_handles = species.call_handles(&audio_assets);

    // Pick a target tree
    let target_tree = tree_positions[rng.random_range(0..tree_positions.len())];

    // Spawn from a random edge outside the visible area
    let angle: f32 = rng.random_range(0.0..std::f32::consts::TAU);
    let spawn_distance = 25.0;
    let spawn_pos = Vec3::new(
        angle.cos() * spawn_distance,
        target_tree.y + rng.random_range(-0.5..1.0),
        angle.sin() * spawn_distance,
    );

    let bird_mesh = meshes.add(Sphere::new(species.radius()).mesh().uv(12, 8));
    let bird_material = materials.add(StandardMaterial {
        base_color: species.color(),
        perceptual_roughness: 0.7,
        ..default()
    });

    let max_trees = rng.random_range(2..=4);

    commands.spawn((
        Mesh3d(bird_mesh),
        MeshMaterial3d(bird_material),
        Transform::from_translation(spawn_pos),
        Bird {
            species,
            trees_visited: 0,
            max_trees,
        },
        BirdState::Approaching {
            target: target_tree,
        },
        PhysicalTranslation(spawn_pos),
        PreviousPhysicalTranslation(spawn_pos),
        Velocity::default(),
        BirdCallHandles(call_handles),
        SpatialAudioEmitter { instances: vec![] },
        SpatialRadius { radius: 30.0 },
    ));

    // Reset spawn timer with random interval
    spawn_timer.timer = Timer::from_seconds(rng.random_range(8.0..15.0), TimerMode::Once);
}

// -- AI --

fn bird_ai(
    mut commands: Commands,
    time: Res<Time>,
    audio: Res<Audio>,
    trees: Query<&Transform, With<Tree>>,
    mut birds: Query<(
        Entity,
        &mut Bird,
        &mut BirdState,
        &mut Velocity,
        &PhysicalTranslation,
        &BirdCallHandles,
        &mut SpatialAudioEmitter,
    )>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    let tree_positions: Vec<Vec3> = trees.iter().map(|t| t.translation).collect();
    if tree_positions.is_empty() {
        return;
    }

    let mut rng = rand::rng();

    for (entity, mut bird, mut state, mut velocity, phys_pos, call_handles, mut emitter) in
        birds.iter_mut()
    {
        match state.as_mut() {
            BirdState::Approaching { target } | BirdState::FlyingToNext { target } => {
                let to_target = *target - phys_pos.0;
                let distance = to_target.length();

                if distance < 0.5 {
                    // Arrived at tree
                    velocity.0 = Vec3::ZERO;
                    let perch_time = rng.random_range(1.0..3.0);
                    *state = BirdState::Perching {
                        timer: Timer::from_seconds(perch_time, TimerMode::Once),
                    };
                } else {
                    let direction = to_target.normalize();
                    // Gentle sine wave on Y for flapping feel
                    let flap_offset =
                        (time.elapsed_secs() * 4.0).sin() * 0.3 * bird.species.speed();
                    velocity.0 =
                        direction * bird.species.speed() + Vec3::new(0.0, flap_offset, 0.0);
                }
            }

            BirdState::Perching { timer } => {
                timer.tick(time.delta());
                velocity.0 = Vec3::ZERO;

                if timer.is_finished() {
                    // Start vocalizing
                    let call_idx = rng.random_range(0..call_handles.0.len());
                    let handle = audio
                        .play(call_handles.0[call_idx].clone())
                        .with_volume(3.0)
                        .handle();
                    emitter.instances.push(handle.clone());
                    commands.entity(entity).insert(ActiveCall(handle));

                    let vocalize_time = rng.random_range(4.0..12.0);
                    *state = BirdState::Vocalizing {
                        timer: Timer::from_seconds(vocalize_time, TimerMode::Once),
                    };
                }
            }

            BirdState::Vocalizing { timer } => {
                timer.tick(time.delta());
                velocity.0 = Vec3::ZERO;

                if timer.is_finished() {
                    // Stop the call
                    for instance_handle in emitter.instances.iter() {
                        if let Some(instance) = audio_instances.get_mut(instance_handle) {
                            instance.stop(AudioTween::default());
                        }
                    }
                    emitter.instances.clear();
                    commands.entity(entity).remove::<ActiveCall>();

                    bird.trees_visited += 1;

                    if bird.trees_visited >= bird.max_trees {
                        // Depart
                        let angle: f32 = rng.random_range(0.0..std::f32::consts::TAU);
                        let depart_target = Vec3::new(
                            angle.cos() * 30.0,
                            rng.random_range(3.0..8.0),
                            angle.sin() * 30.0,
                        );
                        *state = BirdState::Departing {
                            target: depart_target,
                        };
                    } else {
                        // Fly to next tree
                        let next_tree = tree_positions[rng.random_range(0..tree_positions.len())];
                        *state = BirdState::FlyingToNext { target: next_tree };
                    }
                }
            }

            BirdState::Departing { target } => {
                let to_target = *target - phys_pos.0;
                let distance = to_target.length();

                if distance < 1.0 {
                    velocity.0 = Vec3::ZERO;
                    // Will be cleaned up by despawn_distant_birds
                } else {
                    let direction = to_target.normalize();
                    let flap_offset =
                        (time.elapsed_secs() * 4.0).sin() * 0.3 * bird.species.speed();
                    velocity.0 =
                        direction * bird.species.speed() * 1.2 + Vec3::new(0.0, flap_offset, 0.0);
                }
            }
        }
    }
}

// -- Physics --

fn advance_bird_physics(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<
        (
            &mut PhysicalTranslation,
            &mut PreviousPhysicalTranslation,
            &Velocity,
        ),
        With<Bird>,
    >,
) {
    for (mut current, mut previous, velocity) in query.iter_mut() {
        previous.0 = current.0;
        current.0 += velocity.0 * fixed_time.delta_secs();
    }
}

fn interpolate_bird_transforms(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<
        (
            &mut Transform,
            &PhysicalTranslation,
            &PreviousPhysicalTranslation,
        ),
        With<Bird>,
    >,
) {
    let alpha = fixed_time.overstep_fraction();
    for (mut transform, current, previous) in query.iter_mut() {
        transform.translation = previous.0.lerp(current.0, alpha);
    }
}

// -- Cleanup --

fn despawn_distant_birds(
    mut commands: Commands,
    birds: Query<(Entity, &PhysicalTranslation, &BirdState), With<Bird>>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    emitters: Query<&SpatialAudioEmitter>,
) {
    for (entity, phys_pos, state) in birds.iter() {
        if let BirdState::Departing { .. } = state {
            let distance = phys_pos.0.length();
            if distance > 28.0 {
                // Stop any audio before despawning
                if let Ok(emitter) = emitters.get(entity) {
                    for handle in &emitter.instances {
                        if let Some(instance) = audio_instances.get_mut(handle) {
                            instance.stop(AudioTween::default());
                        }
                    }
                }
                commands.entity(entity).despawn();
            }
        }
    }
}
