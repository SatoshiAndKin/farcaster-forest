use crate::GameState;
use crate::loading::AudioAssets;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AudioPlugin, SpatialAudioPlugin))
            .add_systems(OnEnter(GameState::Playing), start_ambient);
    }
}

fn start_ambient(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio
        .play(audio_assets.ambient.clone())
        .looped()
        .with_volume(0.015);
}
