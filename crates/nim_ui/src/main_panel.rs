use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

#[derive(Resource)]
pub struct UiState {}

impl Default for UiState {
    fn default() -> Self {
        UiState {}
    }
}

pub fn ui_system(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Central Panel");
    });

    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        ui.heading("Top Panel");
    });

    egui::TopBottomPanel::top("tab_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut 0, 0, "Tab 1");
            ui.selectable_value(&mut 0, 1, "Tab 2");
            ui.selectable_value(&mut 0, 2, "Tab 3");
        });
    });

    egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        ui.heading("Bottom Panel");
    });

    egui::SidePanel::left("left_panel").show(ctx, |ui| {
        ui.heading("Left Panel");
    });

    egui::SidePanel::right("right_panel").show(ctx, |ui| {
        ui.heading("Right Panel");
    });
}
