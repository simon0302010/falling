use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn detect_collision(
    rapier_context: ReadRapierContext,
    query: Query<(Entity, &Collider, &Transform, &Name, Option<&Velocity>)>,
) {
    if let Ok(context) = rapier_context.single() {
        for (entity, collider, transform, name, vel) in query.iter() {
            let filter = QueryFilter::new().exclude_collider(entity);
            let shape = &*collider.raw;
            context.intersect_shape(
                transform.translation.truncate(),
                0.0,
                shape,
                filter,
                |e| {
                    if let Ok((_, _, _, other_name, other_vel)) = query.get(e) {
                        let impact = vel.map_or(0.0, |v| v.linvel.length()) + other_vel.map_or(0.0, |v| v.linvel.length());
                        println!("Collision: {} with {}, Impact: {:.2}", name.as_str(), other_name.as_str(), impact);
                    }
                    true
                },
            );
        }
    }
}