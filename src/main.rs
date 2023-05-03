use bevy::{
    prelude::*,
    window::{close_on_esc, WindowMode},
};

mod anguila;
mod board;
mod collision;
mod input;
mod segment;
mod targets;

use anguila::{move_anguila, setup_anguila};
use board::init_board;
use collision::target_collision;
use input::handle_input;
use segment::move_segments;
use targets::{spawn_targets, SPAWN_TIME};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Anguila".to_string(),
                mode: WindowMode::BorderlessFullscreen,
                transparent: true,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup_camera)
        .add_startup_system(init_board)
        .add_startup_system(setup_anguila)
        .add_system(move_anguila)
        .add_system(handle_input.before(move_anguila))
        .add_system(target_collision)
        .add_system(move_segments)
        .add_system(spawn_targets.in_schedule(CoreSchedule::FixedUpdate))
        .insert_resource(FixedTime::new_from_secs(SPAWN_TIME))
        .insert_resource(ClearColor(Color::NONE))
        .add_system(close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
