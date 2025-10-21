use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::camera::MainCamera;

pub fn setup_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2d::default(), MainCamera));

    let white_material = materials.add(Color::srgb(1.0, 1.0, 1.0));

    // ground
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(Mesh2d(
            meshes.add(Rectangle::new(1000.0, 100.0))
        ))
        .insert(MeshMaterial2d(white_material.clone()))
        .insert(Transform::from_xyz(0.0, -100.0, 0.0));
}