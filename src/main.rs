use bevy::prelude::*;
use bevy::input::mouse::MouseButton;
use bevy::window::Window;
use bevy::input::ButtonInput;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_ball)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d::default());

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

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
    mouse_input: Res<ButtonInput<MouseButton>>
) {
    for window in windows.iter() {
        if let Some(_cursor_pos) = window.cursor_position(){
            if mouse_input.just_pressed(MouseButton::Left) {
                let ball_radius: f32 = 50.0;
                commands
                    .spawn(RigidBody::Dynamic)
                    .insert(Mesh2d(
                        meshes.add(Circle::new(ball_radius))
                    ))
                    .insert(MeshMaterial2d(
                        materials.add(Color::srgb(1.0, 1.0, 1.0))
                    ))
                    .insert(Collider::ball(ball_radius))
                    .insert(Restitution::coefficient(0.7))
                    .insert(Transform::from_xyz(0.0, 100.0, 0.0));
            }
        }
    }
}