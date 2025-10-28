use bevy::prelude::*;
use crate::player::PlayerData;

#[derive(Component)]
pub struct ScoreText;

pub fn spawn_score_ui(
    mut commands: Commands,
    player_data: ResMut<PlayerData>
) {
    commands.spawn((
        Text::new(format!("Score: {}", player_data.score)),
        TextFont {
            font_size: 20.0,
            ..Default::default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
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
    player_data: Res<PlayerData>
) {
    if let Ok(mut score_text) = score_query.single_mut() {
        score_text.0 = format!("Score: {}", player_data.score);
    }
}