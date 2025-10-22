use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn detect_collision(
    mut contact_force_events: EventReader<ContactForceEvent>,
    name_query: Query<&Name>,
) {
    for contact_force_event in contact_force_events.read() {
        let name1 = name_query.get(contact_force_event.collider1).unwrap();
        let name2 = name_query.get(contact_force_event.collider2).unwrap();

        let impact_strength = contact_force_event.total_force_magnitude as i32;

        if !(name1.contains("player") && name2.contains("player")) {
            println!(
                "Collision between '{}' and '{}'. Strength: {}",
                name1, name2, impact_strength
            )
        }
    }
}