use bevy::asset::Asset;
use bevy::prelude::*;
use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

use crate::game_states::GameState;

#[derive(Resource, Default)]
pub struct JustLoadedTheme(pub bool);

#[derive(Resource, Default)]
pub struct CurrentThemeIndex(pub usize);

#[derive(Resource)]
pub struct ThemeInfo {
    pub loaded: bool,
}

#[derive(Resource, Default)]
pub struct ThemeManifestHandle(pub Handle<ThemeManifest>);

#[derive(Deserialize, Debug, Clone, Serialize, Reflect)]
pub struct ThemeManifestEntry {
    pub path: String,
    pub name: String,
}

#[derive(Asset, Serialize, Deserialize, Clone, Debug, Reflect)]
pub struct ThemeManifest {
    pub themes: Vec<ThemeManifestEntry>,
}

#[derive(Asset, Serialize, Deserialize, Clone, Debug, Reflect)]
pub struct Theme {
    // color for background
    #[serde(default = "default_black")]
    pub background_color: ColorData,
    // color for all ui text
    #[serde(default = "default_white")]
    pub text_color: ColorData,
    // player head color
    #[serde(default = "default_white")]
    pub player_head_color: ColorData,
    // player body color
    #[serde(default = "default_white")]
    pub player_body_color: ColorData,
    // renders obstacles in grayscale if true
    #[serde(default = "default_true")]
    pub obstacles_grayscale: bool,
    // how much randomness is added to the obstacle color. 0.0 is no variation from base, 1.0 is completely random color.
    #[serde(default = "default_color_variation")]
    pub obstacles_color_variation: f64,
    // base color for obstacles. randomness is added afterwards.
    #[serde(default = "default_obstacle_color")]
    pub obstacles_base_color: ColorData,
    // color when body part is broken (default is yellow)
    #[serde(default = "default_broken_color")]
    pub player_broken_color: ColorData,
    // color when broken body part is hit again resulting in the player dying
    #[serde(default = "default_final_color")]
    pub player_final_color: ColorData,
    // path to image texture of player head. is an empty string by default.
    #[serde(default = "default_empty")]
    pub player_head_texture: String,
    // color for the walls that are to the left and right of the player
    #[serde(default = "default_walls_color")]
    pub walls_color: ColorData,
    // path to background music to play
    #[serde(default = "default_empty")]
    pub music_path: String,
    // path to sound for bone break
    #[serde(default = "default_empty")]
    pub bone_break_path: String,
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

pub fn load_themes_manifest(mut commands: Commands, asset_server: Res<AssetServer>) {
    let manifest_handle = ThemeManifestHandle(asset_server.load("themes/manifest.ron"));
    commands.insert_resource(manifest_handle);
}

pub fn cycle_theme(
    kb_input: Res<ButtonInput<KeyCode>>,
    manifest_handle: Res<ThemeManifestHandle>,
    manifests: Res<Assets<ThemeManifest>>,
    mut theme_handle: ResMut<ThemeHandle>,
    mut current_index: ResMut<CurrentThemeIndex>,
    asset_server: Res<AssetServer>,
    mut text_query: Query<&mut Text>,
    mut just_loaded: ResMut<JustLoadedTheme>,
) {
    if kb_input.just_pressed(KeyCode::Tab) {
        if let Some(manifest) = manifests.get(&manifest_handle.0) {
            let themes = &manifest.themes;
            if !themes.is_empty() {
                current_index.0 = (current_index.0 + 1) % themes.len();
                let next_theme_path = &themes[current_index.0].path;
                theme_handle.0 = asset_server.load(next_theme_path);
                info!("Switched to theme: {}", &themes[current_index.0].name);

                for mut text_item in text_query.iter_mut() {
                    if text_item.0.contains("Current Theme") {
                        text_item.0 = format!("Current Theme: {}", &themes[current_index.0].name);
                    }
                }
            }
        }

        just_loaded.0 = true;
    }
}

pub fn update_theme(
    mut materials: ResMut<Assets<ColorMaterial>>,
    theme_handle: Res<ThemeHandle>,
    themes: Res<Assets<Theme>>,
    mut camera_query: Query<&mut Camera>,
    mut text_color_query: Query<&mut TextColor>,
    mut mesh_query: Query<(&Name, &mut MeshMaterial2d<ColorMaterial>)>,
    mut image_node_query: Query<&mut ImageNode>,
    game_state: Res<State<GameState>>,
    asset_server: Res<AssetServer>,
    theme_info: Res<ThemeInfo>,
) {
    if !theme_handle.is_changed()
        && !(game_state.is_changed() && *game_state.get() == GameState::InGame)
        && !theme_info.is_changed()
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

        // hints for keybindings
        for mut image_node in image_node_query.iter_mut() {
            image_node.color = theme.text_color.to_color();
        }

        // player body part color and walls color
        for (part_name, mut mesh_material) in mesh_query.iter_mut() {
            if part_name.as_str() == "player_head" {
                if !theme.player_head_texture.is_empty() {
                    mesh_material.0 = materials
                        .add(asset_server.load(format!("themes/{}", &theme.player_head_texture)))
                } else {
                    mesh_material.0 = materials.add(theme.player_head_color.to_color());
                }
            } else if part_name.as_str().contains("player") {
                mesh_material.0 = materials.add(theme.player_body_color.to_color());
            } else if part_name.as_str() == "wall" {
                mesh_material.0 = materials.add(theme.walls_color.to_color());
            } else if part_name.as_str().contains("obstacle") {
                mesh_material.0 = materials.add(theme.obstacles_base_color.to_color())
            }
        }
    } else {
        warn!("Failed to load theme. Trying again...")
    }
}

pub fn check_theme(
    theme_handle: Res<ThemeHandle>,
    themes: Res<Assets<Theme>>,
    mut theme_info: ResMut<ThemeInfo>,
) {
    if let Some(_) = themes.get(&theme_handle.0) {
        if theme_info.loaded == false {
            theme_info.loaded = true;
        }
    } else {
        if theme_info.loaded == true {
            theme_info.loaded = false;
        }
    }
}

// defaults for theme
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

fn default_obstacle_color() -> ColorData {
    ColorData {
        red: 0.3,
        green: 0.3,
        blue: 0.3,
        alpha: 1.0,
    }
}

fn default_broken_color() -> ColorData {
    ColorData {
        red: 1.0,
        green: 1.0,
        blue: 0.2,
        alpha: 1.0,
    }
}

fn default_final_color() -> ColorData {
    ColorData {
        red: 1.0,
        green: 0.2,
        blue: 0.2,
        alpha: 1.0,
    }
}

fn default_empty() -> String {
    "".to_string()
}

fn default_walls_color() -> ColorData {
    ColorData {
        red: 0.15,
        green: 0.15,
        blue: 0.15,
        alpha: 1.0,
    }
}

pub fn show_current_theme(mut commands: Commands) {
    commands.spawn((
        Text::new("Current Theme: Default"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(10.0),
            top: Val::Px(10.0),
            ..default()
        },
    ));
}
