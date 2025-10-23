use bevy::prelude::*;
use crate::game_states::GameState;
use crate::player::PlayerData;
use crate::player_setup::PlayerBodyPart;

#[derive(Component)]
pub struct GameOverText;

#[derive(Component)]
pub struct PreGameText;

pub fn spawn_game_over_ui(
    mut commands: Commands,
    mut player_data: ResMut<PlayerData>,
) {
    commands.spawn((
        Text::new(format!("{}\nPress Space to restart", player_data.last_death_str)),
        TextFont {
            font_size: 30.0,
            ..Default::default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            top: Val::Percent(45.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        GameOverText,
    ));

    player_data.broken_parts.clear();
}

pub fn despawn_game_over_ui(
    mut commands: Commands,
    query: Query<Entity, With<GameOverText>>,
    player_part_query: Query<Entity, With<PlayerBodyPart>>
) {
    for entity in query.iter() { commands.entity(entity).despawn(); }

    for player_part in player_part_query.iter() {
        commands.entity(player_part).despawn();
    }
}

pub fn handle_game_over_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) { next_state.set(GameState::InGame); }
}

pub fn spawn_pre_game_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("Press Space to start."),
        TextFont {
            font_size: 30.0,
            ..Default::default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            top: Val::Percent(45.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        PreGameText,
    ));
}

pub fn despawn_pre_game_ui(
    mut commands: Commands,
    query: Query<Entity, With<PreGameText>>,
    player_part_query: Query<Entity, With<PlayerBodyPart>>
) {
    for entity in query.iter() { commands.entity(entity).despawn(); }

    for player_part in player_part_query.iter() {
        commands.entity(player_part).despawn();
    }
}

pub fn handle_pre_game_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) { next_state.set(GameState::InGame); }
}