/*
Ideas:
- Turn player limbs red when broken
- Player dies when broken limb experiences too much force
- Player dies instantly when head experiences too much force
*/

use std::collections::HashSet;

use bevy::prelude::*;
use bevy::input::ButtonInput;
use bevy_rapier2d::prelude::*;

use crate::player_setup::{PlayerBodyPart, PlayerTorso};

#[derive(Resource)]
pub struct PlayerData {
    pub broken_parts: HashSet<String>,
    pub alive: bool,
}

pub fn handle_collision(
    mut contact_force_events: EventReader<ContactForceEvent>,
    name_query: Query<&Name>,
    mut player_data: ResMut<PlayerData>,
    mut color_query: Query<&mut MeshMaterial2d<ColorMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let broken_color = Color::srgb(1.0, 0.3, 0.3);

    for contact_force_event in contact_force_events.read() {
        let name1 = name_query.get(contact_force_event.collider1).unwrap();
        let name2 = name_query.get(contact_force_event.collider2).unwrap();

        let impact_force = contact_force_event.total_force_magnitude as i32;

        if !(name1.contains("player") && name2.contains("player")) {
            println!(
                "Collision between '{}' and '{}'. Force: {}",
                name1, name2, impact_force
            );
            if player_data.broken_parts.contains(&name1.to_string()) || player_data.broken_parts.contains(&name2.to_string()) {
                player_data.alive = false;
                println!("Player died.");
                return;
            }
            if name1.contains("player") {
                player_data.broken_parts.insert(name1.to_string());
                if let Ok(material_handle) = color_query.get_mut(contact_force_event.collider1) {
                    if let Some(material) = materials.get_mut(&material_handle.0) {
                        material.color = broken_color.clone();
                    }
                }
            }
            if name2.contains("player") {
                player_data.broken_parts.insert(name2.to_string());
                if let Ok(material_handle) = color_query.get_mut(contact_force_event.collider2) {
                    if let Some(material) = materials.get_mut(&material_handle.0) {
                        material.color = broken_color.clone();
                    }
                }
            }
            if player_data.broken_parts.contains("player_head") {
                player_data.alive = false;
                println!("Player died because head was hit too hard.");
            }
        }
    }
}

const MOVE_ACCELERATION: f32 = 100.0;
const MAX_MOVE_SPEED: f32 = 600.0;

pub fn player_control(
    mut player_query: Query<&mut Velocity, With<PlayerTorso>>,
    kb_input: Res<ButtonInput<KeyCode>>
) {
    if kb_input.pressed(KeyCode::ArrowRight) {
        if let Ok(mut velocity) = player_query.single_mut() {
            if velocity.linvel.x <= MAX_MOVE_SPEED - MOVE_ACCELERATION {
                velocity.linvel.x += MOVE_ACCELERATION;
            } else {
                velocity.linvel.x = MAX_MOVE_SPEED;
            }
        }
    } else if kb_input.pressed(KeyCode::ArrowLeft) {
        if let Ok(mut velocity) = player_query.single_mut() {
            if velocity.linvel.x >= -(MAX_MOVE_SPEED - MOVE_ACCELERATION) {
                velocity.linvel.x -= MOVE_ACCELERATION;
            } else {
                velocity.linvel.x = -MAX_MOVE_SPEED;
            }
        }
    // temporary
    } else if kb_input.just_pressed(KeyCode::ArrowUp) {
        if let Ok(mut velocity) = player_query.single_mut() {
            velocity.linvel.y += 5000.0;
        }
    }
}

const RESET_HEIGHT: f32 = 5000.0;
const MIN_HEIGHT: f32 = -5000.0;

pub fn recenter_world(
    mut transforms: ParamSet<(
        Query<&Transform, With<PlayerTorso>>,
        Query<&mut Transform, With<PlayerBodyPart>>,
    )>
) {
    if let Ok(torso_transform) = transforms.p0().single() {
        let torso_y = torso_transform.translation.y;

        if torso_y < MIN_HEIGHT {
            let diff = RESET_HEIGHT - torso_y;

            for mut rigid_body in transforms.p1().iter_mut() {
                rigid_body.translation.y += diff;
            }
        }
    }
}