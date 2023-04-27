use bevy::prelude::{Input, KeyCode, Query, Res};

use crate::anguila::{Direction, PlayerMovement};

pub fn handle_input(key: Res<Input<KeyCode>>, mut query: Query<&mut PlayerMovement>) {
    let mut anguila = query.single_mut();

    if key.pressed(KeyCode::Left) {
        if key.pressed(KeyCode::Up) {
            anguila.direction = Direction::LeftUp;
            return;
        }
        if key.pressed(KeyCode::Down) {
            anguila.direction = Direction::LeftDown;
            return;
        }

        anguila.direction = Direction::Left;
    }

    if key.pressed(KeyCode::Right) {
        if key.pressed(KeyCode::Up) {
            anguila.direction = Direction::RightUp;
            return;
        }
        if key.pressed(KeyCode::Down) {
            anguila.direction = Direction::RightDown;
            return;
        }

        anguila.direction = Direction::Right;
    }

    if key.pressed(KeyCode::Up) {
        anguila.direction = Direction::Up;
    }

    if key.pressed(KeyCode::Down) {
        anguila.direction = Direction::Down;
    }
}
