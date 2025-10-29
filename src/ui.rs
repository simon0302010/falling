use crate::player::PlayerData;
use bevy::prelude::*;

const WHITE_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);

#[derive(Component)]
pub struct ScoreText;

pub fn spawn_score_ui(mut commands: Commands, player_data: ResMut<PlayerData>) {
    commands.spawn((
        Text::new(format!("Score: {}", player_data.score)),
        TextFont {
            font_size: 20.0,
            ..Default::default()
        },
        TextColor(WHITE_COLOR),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            top: Val::Px(15.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ScoreText,
    ));
}

pub fn update_score_ui(
    mut score_query: Query<&mut Text, With<ScoreText>>,
    player_data: Res<PlayerData>,
) {
    if let Ok(mut score_text) = score_query.single_mut() {
        score_text.0 = format!("Score: {}", player_data.score);
    }
}

pub fn show_keybindings(mut commands: Commands, asset_server: Res<AssetServer>) {
    // left arrow
    commands.spawn((
        ImageNode {
            image: asset_server.load("controls/arrow-left.png"),
            color: Color::srgb(1.0, 1.0, 1.0),
            ..Default::default()
        },
        Node {
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            width: Val::Px(22.0),
            height: Val::Px(22.0),
            ..default()
        },
    ));
    commands.spawn((
        Text::new(": Move Left"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(WHITE_COLOR),
        Node {
            left: Val::Px(37.0),
            top: Val::Px(11.0),
            ..default()
        },
    ));

    // right arrow
    commands.spawn((
        ImageNode {
            image: asset_server.load("controls/arrow-right.png"),
            color: Color::srgb(1.0, 1.0, 1.0),
            ..default()
        },
        Node {
            left: Val::Px(10.0),
            top: Val::Px(40.0),
            width: Val::Px(22.0),
            height: Val::Px(22.0),
            ..default()
        },
    ));
    commands.spawn((
        Text::new(": Move Right"),
        TextFont {
            font_size: 18.0,
            ..Default::default()
        },
        TextColor(WHITE_COLOR),
        Node {
            left: Val::Px(37.0),
            top: Val::Px(41.0),
            ..default()
        },
    ));

    // r key
    commands.spawn((
        ImageNode {
            image: asset_server.load("controls/r.png"),
            color: Color::srgb(1.0, 1.0, 1.0),
            ..default()
        },
        Node {
            left: Val::Px(10.0),
            top: Val::Px(70.0),
            width: Val::Px(22.0),
            height: Val::Px(22.0),
            ..default()
        },
    ));
    commands.spawn((
        Text::new(": Reset"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(WHITE_COLOR),
        Node {
            left: Val::Px(37.0),
            top: Val::Px(71.0),
            ..default()
        },
    ));
}
