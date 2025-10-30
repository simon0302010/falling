use bevy::prelude::*;

use crate::themes::{JustLoadedTheme, Theme, ThemeHandle};

#[derive(Component)]
pub struct BackgroundMusic;

pub fn play_background_audio(mut commands: Commands) {
    commands.spawn((
        AudioPlayer::new(Handle::<AudioSource>::default()),
        PlaybackSettings::LOOP,
        BackgroundMusic,
    ));
}

pub fn update_music(
    themes: Res<Assets<Theme>>,
    theme_handle: Res<ThemeHandle>,
    mut music_query: Query<(&mut AudioPlayer, Option<&AudioSink>), With<BackgroundMusic>>,
    asset_server: Res<AssetServer>,
    mut just_loaded: ResMut<JustLoadedTheme>,
) {
    if just_loaded.0 {
        if let Some(theme) = themes.get(&theme_handle.0) {
            if let Ok((mut music_player, audio_sink)) = music_query.single_mut() {
                if !theme.music_path.is_empty() {
                    music_player.0 = asset_server.load(format!("themes/{}", &theme.music_path));
                    if let Some(sink) = audio_sink {
                        sink.play();
                    }
                    info!("Playing background music.")
                } else {
                    if let Some(sink) = audio_sink {
                        sink.pause();
                    }
                    info!("Stopping background music playback.");
                }
            }
        }
        just_loaded.0 = false;
    }
}
