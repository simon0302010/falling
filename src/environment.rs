use std::{ops::RangeInclusive, time::SystemTime};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{Rng, rngs::StdRng};

use crate::player_setup::PlayerTorso;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct ObstacleObject;

#[derive(Resource)]
pub struct ObstaclesData {
    pub last_spawned: SystemTime,
    pub rng: StdRng,
}

pub fn setup_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let grey_material = materials.add(Color::srgb(0.15, 0.15, 0.15));

    // walls
    commands
        .spawn(Mesh2d(meshes.add(Rectangle::new(20.0, 15000.0))))
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
        .spawn(Mesh2d(meshes.add(Rectangle::new(20.0, 15000.0))))
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
const MIN_OBSTACLE_DISTANCE: f32 = 700.0;
const UNDER_PLAYER_SPAWN: f32 = 1000.0;
const MIN_SPAWN_HEIGHT: f32 = -4000.0;

pub fn manage_obstacles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut obstacles_data: ResMut<ObstaclesData>,
    mut obstacles: Query<(Entity, &Transform), With<ObstacleObject>>,
    player_query: Query<&Transform, With<PlayerTorso>>,
) {
    // TODO: more configurable
    if let Ok(player_transform) = player_query.single() {
        // create new obstacle if conditions are met
        if let Ok(elapsed_time) = obstacles_data.last_spawned.elapsed()
            && elapsed_time.as_millis() > MAX_SPAWN_DELTA_MS
            && obstacles_data.rng.gen_bool(FRAME_OBSTACLE_SPAWN_CHANCE)
        {
            let new_y = player_transform.translation.y - UNDER_PLAYER_SPAWN;

            let min_dist_sq = MIN_OBSTACLE_DISTANCE * MIN_OBSTACLE_DISTANCE;
            let too_close = obstacles.iter().any(|(_, t)| {
                let dy = t.translation.y - new_y;
                (dy * dy) < min_dist_sq
            });

            if !too_close && player_transform.translation.y >= MIN_SPAWN_HEIGHT {
                spawn_random_obstacle(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    -295..=295,
                    120..=180,
                    &mut obstacles_data.rng,
                    new_y,
                );

                obstacles_data.last_spawned = SystemTime::now();
            }
        }

        // delete if out of frame
        for (obstacle_entity, obstacle_transform) in obstacles.iter_mut() {
            if obstacle_transform.translation.y
                >= player_transform.translation.y + OVER_PLAYER_DESPAWN
                || obstacle_transform.translation.y
                    <= player_transform.translation.y - UNDER_PLAYER_SPAWN - 100.0
            {
                commands.entity(obstacle_entity).despawn();
            }
        }
    }
}

fn spawn_random_obstacle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    x_range: RangeInclusive<i32>,
    size_range: RangeInclusive<i32>,
    gen_rng: &mut StdRng,
    y_height: f32,
) {
    let obj_width = gen_rng.gen_range(size_range.clone()) as f32;
    let obj_height = gen_rng.gen_range(size_range.clone()) as f32;
    let x_low = *x_range.start() as f32 + (obj_width / 2.0);
    let x_high = *x_range.end() as f32 - (obj_width / 2.0);
    let new_x = gen_rng.gen_range(x_low..x_high);
    let new_y = y_height;

    // TODO: make configurable (maybe even custom image textures)
    let grayscale = true;
    let color_variation = 0.1;
    let base_color = Vec3::new(0.3, 0.3, 0.3); // x = r, y = g, z = b

    let c_red = gen_rng
        .gen_range(base_color.x - color_variation..base_color.x + color_variation)
        .clamp(0.0, 1.0);
    let c_green = gen_rng
        .gen_range(base_color.y - color_variation..base_color.y + color_variation)
        .clamp(0.0, 1.0);
    let c_blue = gen_rng
        .gen_range(base_color.z - color_variation..base_color.z + color_variation)
        .clamp(0.0, 1.0);
    let obj_color = if grayscale {
        Color::srgb(c_red, c_red, c_red)
    } else {
        Color::srgb(c_red, c_green, c_blue)
    };

    let rotation = Quat::from_rotation_z(gen_rng.gen_range(0.0..360.0));

    match gen_rng.gen_range(0..3) {
        0 => {
            commands
                .spawn(Mesh2d(meshes.add(Rectangle::new(obj_width, obj_height))))
                .insert(MeshMaterial2d(materials.add(obj_color)))
                .insert(ObstacleObject)
                .insert(Collider::cuboid(obj_width / 2.0, obj_height / 2.0))
                .insert(Transform {
                    translation: Vec3 {
                        x: new_x,
                        y: new_y,
                        z: 0.0,
                    },
                    rotation,
                    ..default()
                })
                .insert(Name::new("obstacle_rectangular"))
                .insert(RigidBody::Dynamic)
                .insert(GravityScale(0.1));
        }
        1 => {
            commands
                .spawn(Mesh2d(meshes.add(Circle::new(obj_width / 2.0))))
                .insert(MeshMaterial2d(materials.add(obj_color)))
                .insert(ObstacleObject)
                .insert(Collider::ball(obj_width / 2.0))
                .insert(Transform::from_xyz(new_x, new_y, 0.0))
                .insert(Name::new("obstacle_round"))
                .insert(RigidBody::Dynamic)
                .insert(GravityScale(0.1));
        }
        2 => {
            let point_a = Vec2::new(0.0, obj_height / 2.0);
            let point_b = Vec2::new(-(obj_width / 2.0), -(obj_height / 2.0));
            let point_c = Vec2::new(obj_width / 2.0, -(obj_height / 2.0));

            commands
                .spawn(Mesh2d(
                    meshes.add(Triangle2d::new(point_a, point_b, point_c)),
                ))
                .insert(MeshMaterial2d(materials.add(obj_color)))
                .insert(ObstacleObject)
                .insert(Collider::triangle(point_a, point_b, point_c))
                .insert(Transform {
                    translation: Vec3 {
                        x: new_x,
                        y: new_y,
                        z: 0.0,
                    },
                    rotation,
                    ..default()
                })
                .insert(Name::new("obstacle_triangular"))
                .insert(RigidBody::Dynamic)
                .insert(GravityScale(0.1));
        }
        _ => unreachable!(),
    }
}
