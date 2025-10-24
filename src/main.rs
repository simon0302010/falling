use std::time::SystemTime;
use std::collections::HashSet;

// use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

mod player;
use player::*;

mod camera;
use camera::*;

mod environment;
use environment::*;

mod player_setup;
use player_setup::*;

mod game_states;
use game_states::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Falling".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(FpsOverlayPlugin::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .insert_resource(ObstaclesData {
            last_spawned: SystemTime::now(),
            rng: StdRng::from_entropy()
        })
        .insert_resource(PlayerData {
            broken_parts: HashSet::new(),
            last_death_str: "".to_string(),
            last_y_position: 200.0,
            score: 0,
        })
        .insert_state(GameState::PreGame)
        .add_systems(Startup, setup_environment)
        .add_systems(Startup, setup_player)
        .add_systems(Startup, setup_camera)
        .add_systems(PostStartup, spawn_score_ui)
        .add_systems(OnEnter(GameState::PreGame), spawn_pre_game_ui)
        .add_systems(OnExit(GameState::PreGame), despawn_pre_game_ui)
        .add_systems(OnExit(GameState::PreGame), setup_player)
        .add_systems(Update, handle_pre_game_input.run_if(in_state(GameState::PreGame)))
        .add_systems(Update, player_control.run_if(in_state(GameState::InGame)))
        .add_systems(Update, recenter_world)
        .add_systems(Update, manage_obstacles.run_if(in_state(GameState::InGame)))
        .add_systems(Update, handle_collision.run_if(in_state(GameState::InGame)))
        .add_systems(OnEnter(GameState::GameOver), spawn_game_over_ui)
        .add_systems(OnExit(GameState::GameOver), despawn_game_over_ui)
        .add_systems(OnExit(GameState::GameOver), setup_player)
        .add_systems(Update, handle_game_over_input.run_if(in_state(GameState::GameOver)))
        .add_systems(PostUpdate, increment_score.run_if(in_state(GameState::InGame)))
        .add_systems(PostUpdate, update_score_ui)
        .add_systems(PostUpdate, camera_follow_y)
        .run();
}