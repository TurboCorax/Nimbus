use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiSettings};

// Try to alter the scale factor of the UI to see the effect
static SCALE_FACTOR: f32 = 2.0;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EguiPlugin))
        .add_systems(Update, ui_example_system)
        .add_systems(Update, apply_scaling)
        .run();
}

// This function is copied from the bevy_egui example
// Original source: https://github.com/vladbat00/bevy_egui/blob/main/examples/simple.rs
// Copyright (c) 2021 Vladyslav Batyrenko
// Licensed under the MIT License: https://github.com/vladbat00/bevy_egui/blob/main/LICENSE
fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}

fn apply_scaling(mut ui_scale: Query<&mut EguiSettings>) {
    ui_scale.get_single_mut().unwrap().scale_factor = SCALE_FACTOR;
}

// Another way to scale the UI
fn apply_scaling_1(mut windows: Query<(&mut EguiSettings, &Window), With<PrimaryWindow>>) {
    if let Ok((mut egui_settings, window)) = windows.get_single_mut() {
        egui_settings.scale_factor = SCALE_FACTOR / window.scale_factor();
    }
}

