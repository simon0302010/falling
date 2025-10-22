use bevy::prelude::*;
use bevy::input::ButtonInput;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct PlayerTorso;

#[derive(Component)]
pub struct PlayerBodyPart;

const MOVE_ACCELERATION: f32 = 100.0;
const MAX_MOVE_SPEED: f32 = 600.0;

pub fn player_control(
    mut player_query: Query<&mut Velocity, With<PlayerTorso>>,
    kb_input: Res<ButtonInput<KeyCode>>
) {
    if kb_input.pressed(KeyCode::ArrowRight) {
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
    // temporary
    } else if kb_input.just_pressed(KeyCode::ArrowUp) {
        if let Ok(mut velocity) = player_query.single_mut() {
            velocity.linvel.y += 5000.0;
        }
    }
}

const RESET_HEIGHT: f32 = 5000.0;
const MIN_HEIGHT: f32 = -5000.0;

pub fn recenter_world(
    mut transforms: ParamSet<(
        Query<&Transform, With<PlayerTorso>>,
        Query<&mut Transform, With<PlayerBodyPart>>,
    )>
) {
    if let Ok(torso_transform) = transforms.p0().single() {
        let torso_y = torso_transform.translation.y;

        if torso_y < MIN_HEIGHT {
            let diff = RESET_HEIGHT - torso_y;

            for mut rigid_body in transforms.p1().iter_mut() {
                rigid_body.translation.y += diff;
            }
        }
    }
}

const PLAYER_LINEAR_DAMPING: f32 = 1.0;
const PLAYER_ANGULAR_DAMPING: f32 = 0.5;

pub fn setup_player(
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
        .insert(PlayerTorso)
        .insert(Velocity::default())
        .insert(Damping {
            linear_damping: PLAYER_LINEAR_DAMPING,
            angular_damping: PLAYER_ANGULAR_DAMPING,
        })
        .insert(RigidBody::Dynamic)
        .insert(TransformInterpolation::default())
        .insert(PlayerBodyPart)
        .insert(Name::new("player_torso"))
        .insert(ContactForceEventThreshold(50.0 * 1000000.0))
        .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
        .id();

    let head = commands
        .spawn(Mesh2d(meshes.add(Circle::new(15.0))))
        .insert(MeshMaterial2d(white_material.clone()))
        .insert(Collider::ball(15.0))
        .insert(Transform::from_xyz(0.0, 236.0, 0.0))
        .insert(Damping {
            linear_damping: PLAYER_LINEAR_DAMPING,
            angular_damping: PLAYER_ANGULAR_DAMPING,
        })
        .insert(RigidBody::Dynamic)
        .insert(TransformInterpolation::default())
        .insert(PlayerBodyPart)
        .insert(Name::new("player_head"))
        .insert(ContactForceEventThreshold(60.0 * 1000000.0))
        .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
        .id();

    let arm_r = commands
        .spawn(Mesh2d(meshes.add(Rectangle::new(10.0, 50.0))))
        .insert(MeshMaterial2d(white_material.clone()))
        .insert(Collider::cuboid(5.0, 25.0))
        .insert(Transform::from_xyz(18.0, 195.0, 0.0))
        .insert(Damping {
            linear_damping: PLAYER_LINEAR_DAMPING,
            angular_damping: PLAYER_ANGULAR_DAMPING,
        })
        .insert(RigidBody::Dynamic)
        .insert(TransformInterpolation::default())
        .insert(PlayerBodyPart)
        .insert(Name::new("player_arm_r"))
        .insert(ContactForceEventThreshold(60.0 * 1000000.0))
        .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
        .id();


    let arm_l = commands
        .spawn(Mesh2d(meshes.add(Rectangle::new(10.0, 50.0))))
        .insert(MeshMaterial2d(white_material.clone()))
        .insert(Collider::cuboid(5.0, 25.0))
        .insert(Transform::from_xyz(-18.0, 195.0, 0.0))
        .insert(Damping {
            linear_damping: PLAYER_LINEAR_DAMPING,
            angular_damping: PLAYER_ANGULAR_DAMPING,
        })
        .insert(RigidBody::Dynamic)
        .insert(TransformInterpolation::default())
        .insert(PlayerBodyPart)
        .insert(Name::new("player_arm_l"))
        .insert(ContactForceEventThreshold(60.0 * 1000000.0))
        .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
        .id();

    let leg_r = commands
        .spawn(Mesh2d(meshes.add(Rectangle::new(10.0, 50.0))))
        .insert(MeshMaterial2d(white_material.clone()))
        .insert(Collider::cuboid(5.0, 25.0))
        .insert(Transform::from_xyz(6.0, 155.0, 0.0))
        .insert(Damping {
            linear_damping: PLAYER_LINEAR_DAMPING,
            angular_damping: PLAYER_ANGULAR_DAMPING,
        })
        .insert(RigidBody::Dynamic)
        .insert(TransformInterpolation::default())
        .insert(PlayerBodyPart)
        .insert(Name::new("player_leg_r"))
        .insert(ContactForceEventThreshold(60.0 * 1000000.0))
        .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
        .id();


    let leg_l = commands
        .spawn(Mesh2d(meshes.add(Rectangle::new(10.0, 50.0))))
        .insert(MeshMaterial2d(white_material.clone()))
        .insert(Collider::cuboid(5.0, 25.0))
        .insert(Transform::from_xyz(-6.0, 155.0, 0.0))
        .insert(Damping {
            linear_damping: PLAYER_LINEAR_DAMPING,
            angular_damping: PLAYER_ANGULAR_DAMPING,
        })
        .insert(RigidBody::Dynamic)
        .insert(TransformInterpolation::default())
        .insert(PlayerBodyPart)
        .insert(Name::new("player_leg_l"))
        .insert(ContactForceEventThreshold(60.0 * 1000000.0))
        .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
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