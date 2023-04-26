use bevy::prelude::{Input, KeyCode, Query, Res, Transform, With};

use crate::snake::PlayerMovement;

const ANGUILA_SPEED: f32 = 1.0;
const DIAGONAL_SPEED: f32 = ANGUILA_SPEED * 0.75;

pub fn move_snake(
    key: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<PlayerMovement>>,
) {
    let mut snake = query.single_mut();
    if key.pressed(KeyCode::Left) {
        if key.pressed(KeyCode::Up) || key.pressed(KeyCode::Down) {
            snake.translation.x -= DIAGONAL_SPEED;
        } else {
            snake.translation.x -= ANGUILA_SPEED;
        }
    }
    if key.pressed(KeyCode::Right) {
        if key.pressed(KeyCode::Up) || key.pressed(KeyCode::Down) {
            snake.translation.x += DIAGONAL_SPEED;
        } else {
            snake.translation.x += ANGUILA_SPEED;
        }
    }
    if key.pressed(KeyCode::Up) {
        if key.pressed(KeyCode::Left) || key.pressed(KeyCode::Right) {
            snake.translation.y += DIAGONAL_SPEED;
        } else {
            snake.translation.y += ANGUILA_SPEED;
        }
    }
    if key.pressed(KeyCode::Down) {
        if key.pressed(KeyCode::Left) || key.pressed(KeyCode::Right) {
            snake.translation.y -= DIAGONAL_SPEED;
        } else {
            snake.translation.y -= ANGUILA_SPEED;
        }
    }
}
