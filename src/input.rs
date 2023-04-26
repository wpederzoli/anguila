use bevy::prelude::{Input, KeyCode, Query, Res, Transform, With};

use crate::snake::PlayerMovement;

pub fn move_snake(
    key: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<PlayerMovement>>,
) {
    let mut snake = query.single_mut();
    if key.pressed(KeyCode::Left) {
        if key.pressed(KeyCode::Up) || key.pressed(KeyCode::Down) {
            snake.translation.x -= 0.75;
        } else {
            snake.translation.x -= 1.0;
        }
    }
    if key.pressed(KeyCode::Right) {
        if key.pressed(KeyCode::Up) || key.pressed(KeyCode::Down) {
            snake.translation.x += 0.75;
        } else {
            snake.translation.x += 1.0;
        }
    }
    if key.pressed(KeyCode::Up) {
        if key.pressed(KeyCode::Left) || key.pressed(KeyCode::Right) {
            snake.translation.y += 0.75
        } else {
            snake.translation.y += 1.0;
        }
    }
    if key.pressed(KeyCode::Down) {
        if key.pressed(KeyCode::Left) || key.pressed(KeyCode::Right) {
            snake.translation.y -= 0.75
        } else {
            snake.translation.y -= 1.0;
        }
    }
}
