use bevy::{
    prelude::{default, Color, Commands, Component, Transform, Vec3},
    sprite::{Sprite, SpriteBundle},
};
use rand::Rng;

use crate::board::{BOARD_HEIGHT, BOARD_WIDTH, CELL_SIZE};

pub const SPAWN_TIME: f32 = 2.0;
pub const TARGET_WIDTH: f32 = 5.0;
pub const TARGET_HEIGHT: f32 = 5.0;

#[derive(Component)]
pub struct Target;

pub fn spawn_targets(mut commands: Commands) {
    let spawn = rand::thread_rng().gen_range(0..2);

    if spawn > 0 {
        let mut spawn_x = rand::thread_rng().gen_range(0..BOARD_WIDTH * CELL_SIZE as i32); // TODO: account for window width
        let mut spawn_y = rand::thread_rng().gen_range(0..BOARD_HEIGHT * CELL_SIZE as i32); // TODO: account for window height
        spawn_x = ((spawn_x as i32 / CELL_SIZE as i32) * CELL_SIZE as i32) + CELL_SIZE as i32 / 2;
        spawn_y = ((spawn_y as i32 / CELL_SIZE as i32) * CELL_SIZE as i32) + CELL_SIZE as i32 / 2;

        println!("x: {}, y: {}", spawn_x, spawn_y);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(TARGET_WIDTH, TARGET_HEIGHT, 0.0),
                    translation: Vec3::new(spawn_x as f32, spawn_y as f32, 0.0),
                    ..default()
                },
                ..default()
            },
            Target,
        ));
    }
}
