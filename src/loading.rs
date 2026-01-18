use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<AudioAssets>()
                .load_collection::<TextureAssets>(),
        );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
    // TODO: convert mp3s to ogg
    // TODO: load a whole directory without having to list every single file?
    #[asset(
        path = "audio/Voices of Western Backyard Birds updated 2/01 Western Backyard Birds.mp3"
    )]
    pub western_backyard_birds: Handle<AudioSource>,
    #[asset(path = "audio/Voices of Western Backyard Birds updated 2/02 Mourning Dove Song.mp3")]
    pub mourning_dove_song: Handle<AudioSource>,
    #[asset(
        path = "audio/Voices of Western Backyard Birds updated 2/03 Downy Woodpecker Calls.mp3"
    )]
    pub downy_woodpecker_calls: Handle<AudioSource>,
    #[asset(
        path = "audio/Voices of Western Backyard Birds updated 2/04 Downy Woodpecker Drum.mp3"
    )]
    pub downy_woodpecker_drum: Handle<AudioSource>,
    #[asset(
        path = "audio/Voices of Western Backyard Birds updated 2/05 Northern Flicker Call.mp3"
    )]
    pub northern_flicker_call: Handle<AudioSource>,
    #[asset(
        path = "audio/Voices of Western Backyard Birds updated 2/06 Northern Flicker Call 2.mp3"
    )]
    pub northern_flicker_call_2: Handle<AudioSource>,
    #[asset(
        path = "audio/Voices of Western Backyard Birds updated 2/07 Northern Flicker Drum.mp3"
    )]
    pub northern_flicker_drum: Handle<AudioSource>,
    #[asset(path = "audio/Voices of Western Backyard Birds updated 2/08 Steller's Jay Call.mp3")]
    pub stellers_jay_call: Handle<AudioSource>,
    #[asset(path = "audio/Voices of Western Backyard Birds updated 2/09 Steller's Jay Calls.mp3")]
    pub stellers_jay_calls: Handle<AudioSource>,
    #[asset(
        path = "audio/Voices of Western Backyard Birds updated 2/10 California Scrub-Jay Calls.mp3"
    )]
    pub california_scrub_jay_calls: Handle<AudioSource>,
    #[asset(
        path = "audio/Voices of Western Backyard Birds updated 2/11 Black-capped Chickadee Song.mp3"
    )]
    pub black_capped_chickadee_song: Handle<AudioSource>,
    #[asset(
        path = "audio/Voices of Western Backyard Birds updated 2/12 Black-capped Chickadee Call.mp3"
    )]
    pub black_capped_chickadee_call: Handle<AudioSource>,
    #[asset(
        path = "audio/Voices of Western Backyard Birds updated 2/13 White-breasted Nuthatch Song.mp3"
    )]
    pub white_breasted_nuthatch_song: Handle<AudioSource>,
    #[asset(
        path = "audio/Voices of Western Backyard Birds updated 2/14 White-breasted Nuthatch Call 1.mp3"
    )]
    pub white_breasted_nuthatch_call_1: Handle<AudioSource>,
    #[asset(
        path = "audio/Voices of Western Backyard Birds updated 2/15 White-breasted Nuthatch Call 2.mp3"
    )]
    pub white_breasted_nuthatch_call_2: Handle<AudioSource>,
    #[asset(
        path = "audio/Voices of Western Backyard Birds updated 2/16 White-crowned Sparrow Song 1.mp3"
    )]
    pub white_crowned_sparrow_song_1: Handle<AudioSource>,
    #[asset(
        path = "audio/Voices of Western Backyard Birds updated 2/17 White-crowned Sparrow Song 2.mp3"
    )]
    pub white_crowned_sparrow_song_2: Handle<AudioSource>,
    #[asset(
        path = "audio/Voices of Western Backyard Birds updated 2/18 White-crowned Sparrow Call.mp3"
    )]
    pub white_crowned_sparrow_call: Handle<AudioSource>,
    #[asset(
        path = "audio/Voices of Western Backyard Birds updated 2/19 Red-winged Blackbird Song.mp3"
    )]
    pub red_winged_blackbird_song: Handle<AudioSource>,
    #[asset(
        path = "audio/Voices of Western Backyard Birds updated 2/20 Red-winged Blackbird Calls.mp3"
    )]
    pub red_winged_blackbird_calls: Handle<AudioSource>,
    #[asset(path = "audio/Voices of Western Backyard Birds updated 2/21 Cassin's Finch Song.mp3")]
    pub cassins_finch_song: Handle<AudioSource>,
    #[asset(path = "audio/Voices of Western Backyard Birds updated 2/22 Cassin's Finch Call.mp3")]
    pub cassins_finch_call: Handle<AudioSource>,
    #[asset(path = "audio/Voices of Western Backyard Birds updated 2/23 House Finch Song.mp3")]
    pub house_finch_song: Handle<AudioSource>,
    #[asset(path = "audio/Voices of Western Backyard Birds updated 2/24 House Finch Call.mp3")]
    pub house_finch_call: Handle<AudioSource>,
    #[asset(
        path = "audio/Voices of Western Backyard Birds updated 2/25 Pine Siskin Song, Calls.mp3"
    )]
    pub pine_siskin_song_calls: Handle<AudioSource>,
    #[asset(
        path = "audio/Voices of Western Backyard Birds updated 2/26 American Goldfinch Song, Call.mp3"
    )]
    pub american_goldfinch_song_call: Handle<AudioSource>,
    #[asset(
        path = "audio/Voices of Western Backyard Birds updated 2/27 Evening Grosbeak Calls.mp3"
    )]
    pub evening_grosbeak_calls: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
}
