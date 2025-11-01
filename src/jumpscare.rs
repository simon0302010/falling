use bevy::{prelude::*, window::PrimaryWindow};
use bevy_kira_audio::prelude::*;

#[derive(Component)]
pub struct Jumpscare;

#[derive(Resource)]
pub struct JumpscareActivated {
    activated: bool,
}

pub fn setup_jumpscare(mut commands: Commands) {
    commands.insert_resource(JumpscareActivated { activated: false });
}

pub fn activate_jumpscare(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut jumpscare_act: ResMut<JumpscareActivated>,
    audio_player: Res<Audio>,
) {
    if rand::random::<f32>() > 0.001 {
        return;
    }
    commands.spawn((
        ImageNode {
            image: asset_server.load("img/jumpscare.png"),
            color: Color::srgb(1.0, 1.0, 1.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Jumpscare,
    ));

    audio_player.play(asset_server.load("themes/spooky/jumpscare.mp3"));
    jumpscare_act.activated = true;
    info!("Jumpscare spawned!");
}

pub fn despawn_jumpscare(
    mut commands: Commands,
    kb_input: Res<ButtonInput<KeyCode>>,
    mut jumpscare_act: ResMut<JumpscareActivated>,
    jumpscare_ents: Query<Entity, With<Jumpscare>>,
) {
    if kb_input.just_pressed(KeyCode::Escape) && jumpscare_act.activated {
        for jumpscare_ent in jumpscare_ents.iter() {
            commands.entity(jumpscare_ent).despawn();
        }

        jumpscare_act.activated = false;
    }
}
