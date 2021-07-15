use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

pub struct VirtualInputPlugin;


fn virtual_input(egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx(), |ui| {
        ui.label("world");
    });
}

impl Plugin for VirtualInputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_plugin(EguiPlugin)
        .add_system(virtual_input.system());
    }
}