use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{camera::MainCamera, player::PlayerTorso};

#[derive(Component)]
pub struct Wall;

pub fn setup_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2d::default(), MainCamera));

    let _white_material = materials.add(Color::srgb(1.0, 1.0, 1.0));
    let blue_material = materials.add(Color::srgb(0.0, 0.0, 0.7));

    // ground
    /*
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(Mesh2d(
            meshes.add(Rectangle::new(1000.0, 100.0))
        ))
        .insert(MeshMaterial2d(white_material.clone()))
        .insert(Transform::from_xyz(0.0, -100.0, 0.0));
    */

    // walls
    commands
        .spawn(Mesh2d(
            meshes.add(Rectangle::new(40.0, 4000.0))
        ))
        .insert(Wall)
        .insert(MeshMaterial2d(blue_material.clone()))
        .insert(Collider::cuboid(20.0, 2000.0))
        .insert(Transform::from_xyz(-300.0, 0.0, 0.0));

    commands
        .spawn(Mesh2d(
            meshes.add(Rectangle::new(40.0, 4000.0))
        ))
        .insert(Wall)
        .insert(MeshMaterial2d(blue_material.clone()))
        .insert(Collider::cuboid(20.0, 2000.0))
        .insert(Transform::from_xyz(300.0, 0.0, 0.0));
}

pub fn move_walls(
    mut transforms: ParamSet<(
        Query<&Transform, With<PlayerTorso>>,
        Query<&mut Transform, With<Wall>>,
    )>
) {
    if let Ok(torso_transform) = transforms.p0().single() {
        let torso_y = torso_transform.translation.y;

        for mut wall in transforms.p1().iter_mut() {
            wall.translation.y = torso_y;
        }
    }
}