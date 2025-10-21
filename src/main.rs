use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod player;
use player::*;

mod camera;
use camera::*;

mod environment;
use environment::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(FpsOverlayPlugin::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup_environment)
        .add_systems(Startup, setup_player)
        .add_systems(Update, player_control)
        .add_systems(Update, print_stats)
        .add_systems(Update, move_walls)
        .add_systems(Update, camera_follow_y)
        .run();
}