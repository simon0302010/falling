use std::time::SystemTime;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{rngs::StdRng, Rng};

use crate::player_setup::PlayerTorso;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct ObstacleObject;

#[derive(Resource)]
pub struct ObstaclesData {
    pub last_spawned: SystemTime,
    pub rng: StdRng
}

pub fn setup_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let grey_material = materials.add(Color::srgb(0.15, 0.15, 0.15));
    
    // walls
    commands
        .spawn(Mesh2d(
            meshes.add(Rectangle::new(20.0, 15000.0))
        ))
        .insert(Wall)
        .insert(MeshMaterial2d(grey_material.clone()))
        .insert(Collider::cuboid(10.0, 7500.0))
        .insert(Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(Transform::from_xyz(-300.0, 0.0, 0.0))
        .insert(Name::new("wall"));

    commands
        .spawn(Mesh2d(
            meshes.add(Rectangle::new(20.0, 15000.0))
        ))
        .insert(Wall)
        .insert(MeshMaterial2d(grey_material.clone()))
        .insert(Collider::cuboid(10.0, 7500.0))
        .insert(Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(Transform::from_xyz(300.0, 0.0, 0.0))
        .insert(Name::new("wall"));
}

const MAX_SPAWN_DELTA_MS: u128 = 500;
const FRAME_OBSTACLE_SPAWN_CHANCE: f64 = 0.05;
const OVER_PLAYER_DESPAWN: f32 = 1000.0;
const MIN_OBSTACLE_DISTANCE: f32 = 300.0;
const UNDER_PLAYER_SPAWN: f32 = 1000.0;
const MIN_SPAWN_HEIGHT: f32 = -4000.0;

pub fn manage_obstacles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut obstacles_data: ResMut<ObstaclesData>,
    mut obstacles: Query<(Entity, &Transform), With<ObstacleObject>>,
    player_query: Query<&Transform, With<PlayerTorso>>
) {
    if let Ok(player_transform) = player_query.single() {
        // create new obstacle if conditions are met
        if let Ok(elapsed_time) = obstacles_data.last_spawned.elapsed() {
            if elapsed_time.as_millis() > MAX_SPAWN_DELTA_MS && obstacles_data.rng.gen_bool(FRAME_OBSTACLE_SPAWN_CHANCE) {
                let new_x = obstacles_data.rng.gen_range(-215..215) as f32;
                let new_y = player_transform.translation.y - UNDER_PLAYER_SPAWN;

                // check distance to other obstacles
                let min_dist_sq = MIN_OBSTACLE_DISTANCE * MIN_OBSTACLE_DISTANCE;
                let too_close = obstacles.iter().any(|(_, t)| {
                    let dx = t.translation.x - new_x;
                    let dy = t.translation.y - new_y;
                    (dx * dx + dy * dy) < min_dist_sq
                });

                if !too_close && player_transform.translation.y >= MIN_SPAWN_HEIGHT {
                    commands
                        .spawn(Mesh2d(meshes.add(Rectangle::new(150.0, 100.0))))
                        .insert(MeshMaterial2d(
                            materials.add(Color::srgb(0.3, 0.3, 0.3))
                        ))
                        .insert(ObstacleObject)
                        .insert(Collider::cuboid(75.0, 50.0))
                        .insert(Transform::from_xyz(new_x, new_y, 0.0))
                        .insert(Name::new("obstacle_rectangular"))
                        .insert(RigidBody::Dynamic)
                        .insert(GravityScale(0.1));

                    obstacles_data.last_spawned = SystemTime::now();
                }
            }
        }

        // delete if out of frame
        for (obstacle_entity, obstacle_transform) in obstacles.iter_mut() {
            if obstacle_transform.translation.y >= player_transform.translation.y + OVER_PLAYER_DESPAWN
                || obstacle_transform.translation.y <= player_transform.translation.y - UNDER_PLAYER_SPAWN - 100.0
            {
                commands.entity(obstacle_entity).despawn();
            }
        }
    }
}