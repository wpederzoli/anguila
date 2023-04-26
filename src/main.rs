use bevy::prelude::*;

mod anguila;
mod input;

use anguila::setup_anguila;
use input::move_anguila;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_anguila)
        .add_system(move_anguila)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
