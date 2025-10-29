use bevy::asset::Asset;
use bevy::prelude::*;
use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

use crate::game_states::GameState;
use crate::player_setup::PlayerBodyPart;

fn default_black() -> ColorData {
    ColorData {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
        alpha: 1.0,
    }
}

fn default_white() -> ColorData {
    ColorData {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
        alpha: 1.0,
    }
}

fn default_true() -> bool {
    true
}

fn default_color_variation() -> f64 {
    0.1
}

fn default_gray() -> ColorData {
    ColorData {
        red: 0.3,
        green: 0.3,
        blue: 0.3,
        alpha: 1.0,
    }
}

#[derive(Asset, Serialize, Deserialize, Clone, Debug, Reflect)]
pub struct Theme {
    #[serde(default = "default_black")]
    pub background_color: ColorData,
    #[serde(default = "default_white")]
    pub text_color: ColorData,
    #[serde(default = "default_white")]
    pub player_head_color: ColorData,
    #[serde(default = "default_white")]
    pub player_body_color: ColorData,
    #[serde(default = "default_true")]
    pub obstacles_grayscale: bool,
    #[serde(default = "default_color_variation")]
    pub obstacles_color_variation: f64,
    #[serde(default = "default_gray")]
    pub obstacles_base_color: ColorData,
}

#[derive(Deserialize, Debug, Clone, Serialize, Reflect)]
pub struct ColorData {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl ColorData {
    pub fn to_color(&self) -> Color {
        Color::srgba(self.red, self.green, self.blue, self.alpha)
    }
    pub fn to_vec(&self) -> Vec3 {
        Vec3 {
            x: self.red,
            y: self.green,
            z: self.blue,
        }
    }
}

#[derive(Resource, Default)]
pub struct ThemeHandle(pub Handle<Theme>);

pub fn load_theme(mut commands: Commands, asset_server: Res<AssetServer>) {
    let theme_handle = ThemeHandle(asset_server.load("themes/default.theme.ron"));
    commands.insert_resource(theme_handle);
}

pub fn update_theme(
    mut materials: ResMut<Assets<ColorMaterial>>,
    theme_handle: Res<ThemeHandle>,
    themes: Res<Assets<Theme>>,
    mut camera_query: Query<&mut Camera>,
    mut text_color_query: Query<&mut TextColor>,
    mut player_color_query: Query<
        (&Name, &mut MeshMaterial2d<ColorMaterial>),
        With<PlayerBodyPart>,
    >,
    game_state: Res<State<GameState>>,
) {
    if !theme_handle.is_changed()
        && !(game_state.is_changed() && *game_state.get() == GameState::InGame)
    {
        return;
    }

    info!("Reloading theme...");

    if let Some(theme) = themes.get(&theme_handle.0) {
        // camera clear color
        for mut camera in camera_query.iter_mut() {
            camera.clear_color =
                bevy::render::camera::ClearColorConfig::Custom(theme.background_color.to_color());
        }
        // text color
        for mut text_color in text_color_query.iter_mut() {
            text_color.0 = theme.text_color.to_color();
        }
        // player body part color
        for (part_name, mut mesh_material) in player_color_query.iter_mut() {
            if part_name.as_str() == "player_head" {
                mesh_material.0 = materials.add(theme.player_head_color.to_color());
            } else if part_name.as_str().contains("player") {
                mesh_material.0 = materials.add(theme.player_body_color.to_color());
            }
        }
    } else {
        error!("Failed to load theme. Falling back to default.")
    }
}
