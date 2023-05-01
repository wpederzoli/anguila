use bevy::{prelude::*, window::close_on_esc};

mod anguila;
mod board;
mod collision;
mod input;
mod targets;

use anguila::{move_anguila, setup_anguila};
use board::init_board;
use collision::target_collision;
use input::handle_input;
use targets::{spawn_targets, SPAWN_TIME};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Anguila".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup_camera)
        .add_startup_system(init_board)
        .add_startup_system(setup_anguila)
        .add_system(move_anguila)
        .add_system(handle_input)
        .add_system(close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
