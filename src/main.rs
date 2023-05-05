use bevy::{
    prelude::*,
    sprite::Material2dPlugin,
    window::{close_on_esc, WindowMode},
};

mod anguila;
mod board;
mod collision;
mod input;
mod segment;
mod targets;
mod water;

use anguila::{move_anguila, setup_anguila};
use board::init_board;
use collision::target_collision;
use input::handle_input;
use segment::move_segments;
use targets::{spawn_targets, SPAWN_TIME};
use water::{setup_water, update_water, WaterMaterial};

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
        .add_startup_system(setup_water)
        .add_plugin(Material2dPlugin::<WaterMaterial>::default())
        .add_system(move_anguila)
        .add_system(handle_input.before(move_anguila))
        .add_system(target_collision)
        .add_system(move_segments)
        .add_system(spawn_targets.in_schedule(CoreSchedule::FixedUpdate))
        .add_system(update_water)
        .insert_resource(FixedTime::new_from_secs(SPAWN_TIME))
        .insert_resource(ClearColor(Color::NONE))
        .insert_resource(Time::default())
        .add_system(close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
