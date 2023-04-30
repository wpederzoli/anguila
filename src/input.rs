use bevy::prelude::{Input, KeyCode, Query, Res, With};

use crate::anguila::{Anguila, Direction, MoveDirection};

pub fn handle_input(key: Res<Input<KeyCode>>, mut player: Query<&mut Direction, With<Anguila>>) {
    let mut direction = player.single_mut();

    if key.pressed(KeyCode::Left) {
        if key.pressed(KeyCode::Up) {
            direction.0 = MoveDirection::LeftUp;
            return;
        }
        if key.pressed(KeyCode::Down) {
            direction.0 = MoveDirection::LeftDown;
            return;
        }

        direction.0 = MoveDirection::Left;
    }

    if key.pressed(KeyCode::Right) {
        if key.pressed(KeyCode::Up) {
            direction.0 = MoveDirection::RightUp;
            return;
        }
        if key.pressed(KeyCode::Down) {
            direction.0 = MoveDirection::RightDown;
            return;
        }

        direction.0 = MoveDirection::Right;
    }

    if key.pressed(KeyCode::Up) {
        direction.0 = MoveDirection::Up;
    }

    if key.pressed(KeyCode::Down) {
        direction.0 = MoveDirection::Down;
    }
}
