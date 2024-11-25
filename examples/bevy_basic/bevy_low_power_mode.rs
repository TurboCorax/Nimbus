/// Reference link: https://github.com/bevyengine/bevy/blob/main/examples/window/low_power.rs
use bevy::prelude::*;
use bevy::winit::WinitSettings;


fn main() {
    App::new()
        .insert_resource(WinitSettings::game())
        // Power-saving reactive rendering for applications.
        .insert_resource(WinitSettings::desktop_app())
        .run();
}
