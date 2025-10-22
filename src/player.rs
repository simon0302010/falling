use bevy::prelude::*;
use bevy::input::ButtonInput;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct PlayerTorso;

#[derive(Resource)]
pub struct RecenterTimer(pub Timer);

const MOVE_ACCELERATION: f32 = 100.0;
const MAX_MOVE_SPEED: f32 = 400.0;
const JUMP_VELOCITY: f32 = 2000.0;

pub fn player_control(
    mut player_query: Query<&mut Velocity, With<PlayerTorso>>,
    kb_input: Res<ButtonInput<KeyCode>>
) {
    if kb_input.just_pressed(KeyCode::ArrowUp) {
        if let Ok(mut velocity) = player_query.single_mut() {
            velocity.linvel.y = JUMP_VELOCITY;
        }
    } else if kb_input.pressed(KeyCode::ArrowRight) {
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
    }
}

const RESET_HEIGHT: f32 = 3000.0;

pub fn recenter_world(
    time: Res<Time>,
    mut timer: ResMut<RecenterTimer>,
    mut transforms: ParamSet<(
        Query<&Transform, With<PlayerTorso>>,
        Query<&mut Transform, With<RigidBody>>,
    )>
) {
    if timer.0.tick(time.delta()).just_finished() {
        if let Ok(torso_transform) = transforms.p0().single() {
            let diff = RESET_HEIGHT - torso_transform.translation.y;

            for mut rigid_body in transforms.p1().iter_mut() {
                rigid_body.translation.y += diff;
            }
        }
    }
}

pub fn print_stats(
    velocity_query: Query<&Velocity, With<PlayerTorso>>,
    pos_query: Query<&Transform, With<PlayerTorso>>
) {
    if let Ok(velocity) = velocity_query.single() {
        println!("Player Velocity: ({}, {})", velocity.linvel.x as i32, velocity.linvel.y as i32)
    }

    if let Ok(player_transform) = pos_query.single() {
        println!("Player Position: ({}, {})", player_transform.translation.x as i32, player_transform.translation.y as i32)
    }
}

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let linear_damping = 0.8;
    let angular_damping = 0.5;
    let white_material = materials.add(Color::srgb(1.0, 1.0, 1.0));

    let torso = commands
        .spawn(Mesh2d(meshes.add(Rectangle::new(20.0, 40.0))))
        .insert(MeshMaterial2d(white_material.clone()))
        .insert(Collider::cuboid(10.0, 20.0))
        .insert(Transform::from_xyz(0.0, 200.0, 0.0))
        .insert(PlayerTorso)
        .insert(Velocity::default())
        .insert(Damping {
            linear_damping,
            angular_damping,
        })
        .insert(RigidBody::Dynamic)
        .insert(TransformInterpolation::default())
        .id();

    let head = commands
        .spawn(Mesh2d(meshes.add(Circle::new(15.0))))
        .insert(MeshMaterial2d(white_material.clone()))
        .insert(Collider::ball(15.0))
        .insert(Transform::from_xyz(0.0, 236.0, 0.0))
        .insert(Damping {
            linear_damping,
            angular_damping,
        })
        .insert(RigidBody::Dynamic)
        .insert(TransformInterpolation::default())
        .id();

    let arm_r = commands
        .spawn(Mesh2d(meshes.add(Rectangle::new(10.0, 50.0))))
        .insert(MeshMaterial2d(white_material.clone()))
        .insert(Collider::cuboid(5.0, 25.0))
        .insert(Transform::from_xyz(18.0, 195.0, 0.0))
        .insert(Damping {
            linear_damping,
            angular_damping,
        })
        .insert(RigidBody::Dynamic)
        .insert(TransformInterpolation::default())
        .id();


    let arm_l = commands
        .spawn(Mesh2d(meshes.add(Rectangle::new(10.0, 50.0))))
        .insert(MeshMaterial2d(white_material.clone()))
        .insert(Collider::cuboid(5.0, 25.0))
        .insert(Transform::from_xyz(-18.0, 195.0, 0.0))
        .insert(Damping {
            linear_damping,
            angular_damping,
        })
        .insert(RigidBody::Dynamic)
        .insert(TransformInterpolation::default())
        .id();

    let leg_r = commands
        .spawn(Mesh2d(meshes.add(Rectangle::new(10.0, 50.0))))
        .insert(MeshMaterial2d(white_material.clone()))
        .insert(Collider::cuboid(5.0, 25.0))
        .insert(Transform::from_xyz(6.0, 155.0, 0.0))
        .insert(Damping {
            linear_damping,
            angular_damping,
        })
        .insert(RigidBody::Dynamic)
        .insert(TransformInterpolation::default())
        .id();


    let leg_l = commands
        .spawn(Mesh2d(meshes.add(Rectangle::new(10.0, 50.0))))
        .insert(MeshMaterial2d(white_material.clone()))
        .insert(Collider::cuboid(5.0, 25.0))
        .insert(Transform::from_xyz(-6.0, 155.0, 0.0))
        .insert(Damping {
            linear_damping,
            angular_damping,
        })
        .insert(RigidBody::Dynamic)
        .insert(TransformInterpolation::default())
        .id();

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
                .limits([-5.0, 5.0])
        ))
        .insert(ChildOf(torso));

    // left arm and torso
    commands
        .spawn(ImpulseJoint::new(
            arm_l,
            RevoluteJointBuilder::new()
                .local_anchor1(Vec2::new(5.0, 20.0))
                .local_anchor2(Vec2::new(-13.0, 15.0))
                .limits([-5.0, 5.0])
        ))
        .insert(ChildOf(torso));

    // right leg and torso
    commands
        .spawn(ImpulseJoint::new(
            leg_r,
            RevoluteJointBuilder::new()
                .local_anchor1(Vec2::new(0.0, 25.0))
                .local_anchor2(Vec2::new(6.0, -20.0))
                .limits([-2.0, 2.0])
        ))
        .insert(ChildOf(torso));

    // left leg and torso
    commands
        .spawn(ImpulseJoint::new(
            leg_l,
            RevoluteJointBuilder::new()
                .local_anchor1(Vec2::new(0.0, 25.0))
                .local_anchor2(Vec2::new(-6.0, -20.0))
                .limits([-2.0, 2.0])
        ))
        .insert(ChildOf(torso));
}

// TODO: teleport everything back to (0, 0) every one in a while