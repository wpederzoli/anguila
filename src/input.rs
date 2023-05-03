use bevy::prelude::{Input, KeyCode, Query, Res, With};

use crate::anguila::{Anguila, MoveDirection, Position};

pub fn handle_input(key: Res<Input<KeyCode>>, mut player: Query<&mut Position, With<Anguila>>) {
    let mut direction = player.single_mut();

    if key.pressed(KeyCode::Left) {
        direction.1 = MoveDirection::Left;
    }

    if key.pressed(KeyCode::Right) {
        direction.1 = MoveDirection::Right;
    }

    if key.pressed(KeyCode::Up) {
        direction.1 = MoveDirection::Up;
    }

    if key.pressed(KeyCode::Down) {
        direction.1 = MoveDirection::Down;
    }
}
