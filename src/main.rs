use bevy::prelude::*;
use bevy::input::mouse::MouseButton;
use bevy::window::Window;
use bevy::input::ButtonInput;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup_environment)
        .add_systems(Startup, setup_ragdoll)
        .add_systems(Update, spawn_ball)
        .add_systems(Update, camera_follow_y)
        .run();
}

#[derive(Component)]
struct Torso;

#[derive(Component)]
struct MainCamera;

fn setup_environment(
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

fn setup_ragdoll(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let white_material = materials.add(Color::srgb(1.0, 1.0, 1.0));

    let torso = commands
        .spawn(Mesh2d(meshes.add(Rectangle::new(20.0, 40.0))))
        .insert(MeshMaterial2d(white_material.clone()))
        .insert(Collider::cuboid(10.0, 20.0))
        .insert(Transform::from_xyz(0.0, 200.0, 0.0))
        .insert(Torso)
        .insert(RigidBody::Dynamic).id();

    let head = commands
        .spawn(Mesh2d(meshes.add(Circle::new(15.0))))
        .insert(MeshMaterial2d(white_material.clone()))
        .insert(Collider::ball(15.0))
        .insert(Transform::from_xyz(0.0, 236.0, 0.0))
        .insert(RigidBody::Dynamic).id();

    let arm_r = commands
        .spawn(Mesh2d(meshes.add(Rectangle::new(10.0, 50.0))))
        .insert(MeshMaterial2d(white_material.clone()))
        .insert(Collider::cuboid(5.0, 25.0))
        .insert(Transform::from_xyz(18.0, 195.0, 0.0))
        .insert(RigidBody::Dynamic).id();


    let arm_l = commands
        .spawn(Mesh2d(meshes.add(Rectangle::new(10.0, 50.0))))
        .insert(MeshMaterial2d(white_material.clone()))
        .insert(Collider::cuboid(5.0, 25.0))
        .insert(Transform::from_xyz(-18.0, 195.0, 0.0))
        .insert(RigidBody::Dynamic).id();

    // head and torso
    commands
        .spawn(ImpulseJoint::new(
            head,
            RevoluteJointBuilder::new()
                .local_anchor1(Vec2::new(0.0, -16.0))
                .local_anchor2(Vec2::new(0.0, 20.0))
                .limits([-0.5, 0.5])
        ))
        .insert(ChildOf(torso));

    // right arm and torso
    commands
        .spawn(ImpulseJoint::new(
            arm_r,
            RevoluteJointBuilder::new()
                .local_anchor1(Vec2::new(-5.0, 20.0))
                .local_anchor2(Vec2::new(13.0, 15.0))
                .limits([-2.0, 2.0])
        ))
        .insert(ChildOf(torso));

    // left arm and torso
    commands
        .spawn(ImpulseJoint::new(
            arm_l,
            RevoluteJointBuilder::new()
                .local_anchor1(Vec2::new(5.0, 20.0))
                .local_anchor2(Vec2::new(-13.0, 15.0))
                .limits([-2.0, 2.0])
        ))
        .insert(ChildOf(torso));
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

fn camera_follow_y(
    mut transforms: ParamSet<(
        Query<&Transform, With<Torso>>,
        Query<&mut Transform, With<MainCamera>>,
    )>
) {
    let torso_y = if let Ok(torso) = transforms.p0().single() {
        torso.translation.y
    } else {
        return;
    };
    
    if let Ok(mut camera_transform) = transforms.p1().single_mut() {
        camera_transform.translation.y = torso_y;
    }
}