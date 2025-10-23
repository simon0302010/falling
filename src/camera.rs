use bevy::prelude::*;

use crate::player_setup::PlayerTorso;

#[derive(Component)]
pub struct MainCamera;

pub fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn((Camera2d::default(), MainCamera));
}

pub fn camera_follow_y(
    mut queries: ParamSet<(
        Query<&Transform, With<PlayerTorso>>,
        Query<&mut Transform, With<MainCamera>>,
        Query<&Window>
    )>,
) {
    let torso_y = if let Ok(torso) = queries.p0().single() {
        torso.translation.y
    } else {
        return;
    };

    let window_height = if let Ok(window) = queries.p2().single() {
        window.height()
    } else {
        return;
    };

    if let Ok(mut camera_transform) = queries.p1().single_mut() {
        camera_transform.translation.y = torso_y - window_height / 2.0 + 150.0;
    }
}