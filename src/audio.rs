use bevy::prelude::*;

#[derive(Component)]
pub struct BackgroundMusic;

pub fn play_background_audio(mut commands: Commands) {
    commands.spawn((
        AudioPlayer::new(Handle::<AudioSource>::default()),
        PlaybackSettings::LOOP,
        BackgroundMusic,
    ));
}
