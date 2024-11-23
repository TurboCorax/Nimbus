use bevy::DefaultPlugins;
use bevy::{
    prelude::*,
    winit::WinitSettings,
};
use bevy_egui::EguiPlugin;

fn main() {
    App::new()
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .run();
}