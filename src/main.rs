use bevy::prelude::*;

mod anguila;
mod collision;
mod input;
mod segment;
mod targets;

use anguila::{move_anguila, setup_anguila};
use collision::target_collision;
use input::handle_input;
use segment::{move_segments, update_segments, Segments};
use targets::{spawn_targets, SPAWN_TIME};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Segments(Vec::new()))
        .add_startup_system(setup_camera)
        .add_startup_system(setup_anguila)
        .add_systems((
            handle_input,
            target_collision,
            update_segments,
            move_segments,
            move_anguila,
        ))
        .add_system(bevy::window::close_on_esc)
        .add_system(spawn_targets.in_schedule(CoreSchedule::FixedUpdate))
        .insert_resource(FixedTime::new_from_secs(SPAWN_TIME))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
