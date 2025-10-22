use std::time::SystemTime;

use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::rngs::StdRng;

mod player;
use player::*;

mod camera;
use camera::*;

mod environment;
use environment::*;
use rand::SeedableRng;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(FpsOverlayPlugin::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .insert_resource(ObstaclesData {
            last_spawned: SystemTime::now(),
            rng: StdRng::from_entropy()
        })
        .add_systems(Startup, setup_environment)
        .add_systems(Startup, setup_player)
        .add_systems(Update, player_control)
        .add_systems(Update, print_stats)
        .add_systems(Update, recenter_world)
        .add_systems(Update, manage_obstacles)
        .add_systems(PostUpdate, camera_follow_y)
        .run();
}