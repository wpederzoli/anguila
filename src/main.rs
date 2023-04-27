use bevy::prelude::*;

mod anguila;
mod input;
mod targets;

use anguila::{setup_anguila, target_collision};
use input::move_anguila;
use targets::{spawn_targets, SPAWN_TIME};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_anguila)
        .add_system(move_anguila)
        .add_system(target_collision)
        .add_system(bevy::window::close_on_esc)
        .add_system(spawn_targets.in_schedule(CoreSchedule::FixedUpdate))
        .insert_resource(FixedTime::new_from_secs(SPAWN_TIME))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
