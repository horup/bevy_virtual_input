use bevy::prelude::*;
use bevy_virtual_input::VirtualInputPlugin;

pub fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(VirtualInputPlugin)
        .run();
}