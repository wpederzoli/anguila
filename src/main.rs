use bevy::prelude::*;

mod input;
mod snake;

use input::move_snake;
use snake::setup_snake;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_snake)
        .add_system(move_snake)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
