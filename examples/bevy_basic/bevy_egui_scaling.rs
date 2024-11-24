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
// Licensed under the MIT License: https://github.com/vladbat00/bevy_egui/blob/main/LICENSE

///MIT License
//
// Copyright (c) 2021 Vladyslav Batyrenko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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

