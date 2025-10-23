use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    PreGame,
    InGame,
    GameOver,
}