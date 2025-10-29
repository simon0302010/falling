use bevy::prelude::*;
use bevy::asset::Asset;
use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Asset, Serialize, Deserialize, Clone, Debug, Reflect)]
pub struct Theme {
    pub background_color: ColorData,
    pub text_color: ColorData,
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
}

#[derive(Resource, Default)]
pub struct ThemeHandle(pub Handle<Theme>);

pub fn load_theme(mut commands: Commands, asset_server: Res<AssetServer>) {
    let theme_handle = ThemeHandle(asset_server.load("themes/default.theme.ron"));
    commands.insert_resource(theme_handle);
}

pub fn update_theme(
    theme_handle: Res<ThemeHandle>,
    themes: Res<Assets<Theme>>,
    mut camera_query: Query<&mut Camera>,
) {
    if !theme_handle.is_changed() {
        return;
    }
    if let Some(theme) = themes.get(&theme_handle.0) {
        for mut camera in camera_query.iter_mut() {
            camera.clear_color = bevy::render::camera::ClearColorConfig::Custom(theme.background_color.to_color());
        }
    }
}