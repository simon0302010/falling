// Add to Cargo.toml:
// bevy = "0.16"
// bevy_rapier2d = "0.31"

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_ragdoll_on_click)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    
    // Create ground
    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(800.0, 20.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -300.0, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(400.0, 10.0),
    ));
}

fn spawn_ragdoll_on_click(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Ok(window) = window_query.get_single() {
            if let Some(cursor_pos) = window.cursor_position() {
                if let Ok((camera, camera_transform)) = camera_query.get_single() {
                    if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                        spawn_ragdoll(&mut commands, world_pos);
                    }
                }
            }
        }
    }
}

fn spawn_ragdoll(commands: &mut Commands, pos: Vec2) {
    let head_size = 15.0;
    let body_width = 12.0;
    let body_height = 30.0;
    let limb_width = 8.0;
    let limb_length = 25.0;
    
    // Head
    let head = commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.8, 0.7),
            custom_size: Some(Vec2::new(head_size * 2.0, head_size * 2.0)),
            ..default()
        },
        Transform::from_translation(pos.extend(0.0)),
        RigidBody::Dynamic,
        Collider::ball(head_size),
        ColliderMassProperties::Density(1.0),
    )).id();
    
    // Torso
    let torso = commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.4, 0.8),
            custom_size: Some(Vec2::new(body_width * 2.0, body_height * 2.0)),
            ..default()
        },
        Transform::from_translation((pos + Vec2::new(0.0, -head_size - body_height)).extend(0.0)),
        RigidBody::Dynamic,
        Collider::cuboid(body_width, body_height),
        ColliderMassProperties::Density(2.0),
    )).id();
    
    // Left Upper Arm
    let left_upper_arm = commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.8, 0.7),
            custom_size: Some(Vec2::new(limb_width * 2.0, limb_length * 2.0)),
            ..default()
        },
        Transform::from_translation((pos + Vec2::new(-body_width - limb_width, -head_size - 10.0)).extend(0.0)),
        RigidBody::Dynamic,
        Collider::cuboid(limb_width, limb_length),
        ColliderMassProperties::Density(1.0),
    )).id();
    
    // Right Upper Arm
    let right_upper_arm = commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.8, 0.7),
            custom_size: Some(Vec2::new(limb_width * 2.0, limb_length * 2.0)),
            ..default()
        },
        Transform::from_translation((pos + Vec2::new(body_width + limb_width, -head_size - 10.0)).extend(0.0)),
        RigidBody::Dynamic,
        Collider::cuboid(limb_width, limb_length),
        ColliderMassProperties::Density(1.0),
    )).id();
    
    // Left Upper Leg
    let left_upper_leg = commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.2, 0.6),
            custom_size: Some(Vec2::new(limb_width * 2.0, limb_length * 2.0)),
            ..default()
        },
        Transform::from_translation((pos + Vec2::new(-body_width / 2.0, -head_size - body_height * 2.0 - limb_length)).extend(0.0)),
        RigidBody::Dynamic,
        Collider::cuboid(limb_width, limb_length),
        ColliderMassProperties::Density(1.5),
    )).id();
    
    // Right Upper Leg
    let right_upper_leg = commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.2, 0.6),
            custom_size: Some(Vec2::new(limb_width * 2.0, limb_length * 2.0)),
            ..default()
        },
        Transform::from_translation((pos + Vec2::new(body_width / 2.0, -head_size - body_height * 2.0 - limb_length)).extend(0.0)),
        RigidBody::Dynamic,
        Collider::cuboid(limb_width, limb_length),
        ColliderMassProperties::Density(1.5),
    )).id();
    
    // Fix 3 & 4: Create joints correctly
    // The joint data goes in ImpulseJoint::new(), not in a separate spawn
    
    // Head to Torso (neck)
    commands.spawn(
        ImpulseJoint::new(
            head,
            RevoluteJointBuilder::new()
                .local_anchor1(Vec2::new(0.0, -head_size))
                .local_anchor2(Vec2::new(0.0, body_height))
                .limits([-0.5, 0.5])
        )
    ).insert(RigidBody::Dynamic).insert(Transform::default()).set_parent(torso);
    
    // Left Arm to Torso (shoulder)
    commands.spawn(
        ImpulseJoint::new(
            left_upper_arm,
            RevoluteJointBuilder::new()
                .local_anchor1(Vec2::new(limb_width, limb_length))
                .local_anchor2(Vec2::new(-body_width, body_height - 10.0))
                .limits([-2.0, 2.0])
        )
    ).set_parent(torso);
    
    // Right Arm to Torso (shoulder)
    commands.spawn(
        ImpulseJoint::new(
            right_upper_arm,
            RevoluteJointBuilder::new()
                .local_anchor1(Vec2::new(-limb_width, limb_length))
                .local_anchor2(Vec2::new(body_width, body_height - 10.0))
                .limits([-2.0, 2.0])
        )
    ).set_parent(torso);
    
    // Left Leg to Torso (hip)
    commands.spawn(
        ImpulseJoint::new(
            left_upper_leg,
            RevoluteJointBuilder::new()
                .local_anchor1(Vec2::new(0.0, limb_length))
                .local_anchor2(Vec2::new(-body_width / 2.0, -body_height))
                .limits([-1.5, 1.5])
        )
    ).set_parent(torso);
    
    // Right Leg to Torso (hip)
    commands.spawn(
        ImpulseJoint::new(
            right_upper_leg,
            RevoluteJointBuilder::new()
                .local_anchor1(Vec2::new(0.0, limb_length))
                .local_anchor2(Vec2::new(body_width / 2.0, -body_height))
                .limits([-1.5, 1.5])
        )
    ).set_parent(torso);
}