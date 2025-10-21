use bevy::prelude::*;

use crate::player::PlayerTorso;

#[derive(Component)]
pub struct MainCamera;

pub fn camera_follow_y(
    mut transforms: ParamSet<(
        Query<&Transform, With<PlayerTorso>>,
        Query<&mut Transform, With<MainCamera>>,
    )>
) {
    let torso_y = if let Ok(torso) = transforms.p0().single() {
        torso.translation.y
    } else {
        return;
    };
    
    if let Ok(mut camera_transform) = transforms.p1().single_mut() {
        camera_transform.translation.y = torso_y;
    }
}