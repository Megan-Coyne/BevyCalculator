mod component;
mod constants;
mod evaluate;
mod button_system;
mod clicked_buttons;
mod setup;

use bevy::prelude::*;
use bevy::winit::WinitSettings;
use button_system::button_system;
use setup::setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(clicked_buttons::ClickedButtons::default())
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .run();
}
