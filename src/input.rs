use bevy::prelude::{Input, KeyCode, Query, Res, Transform, Vec2, With};

use crate::{
    anguila::{Anguila, Direction, MoveDirection},
    segment::Segment,
};

pub fn handle_input(
    key: Res<Input<KeyCode>>,
    mut player: Query<(&mut Direction, &Transform), With<Anguila>>,
    mut segments: Query<&mut Segment>,
) {
    let (mut direction, transform) = player.single_mut();

    if key.pressed(KeyCode::Left) {
        if key.pressed(KeyCode::Up) {
            if let Some(mut s) = segments.iter_mut().next() {
                s.0 = Vec2::new(transform.translation.x, transform.translation.y);
                s.1 = direction.0;
            }
            direction.0 = MoveDirection::LeftUp;
            return;
        }
        if key.pressed(KeyCode::Down) {
            if let Some(mut s) = segments.iter_mut().next() {
                s.0 = Vec2::new(transform.translation.x, transform.translation.y);
                s.1 = direction.0;
            }
            direction.0 = MoveDirection::LeftDown;
            return;
        }

        if let Some(mut s) = segments.iter_mut().next() {
            s.0 = Vec2::new(transform.translation.x, transform.translation.y);
            s.1 = direction.0;
        }
        direction.0 = MoveDirection::Left;
    }

    if key.pressed(KeyCode::Right) {
        if key.pressed(KeyCode::Up) {
            if let Some(mut s) = segments.iter_mut().next() {
                s.0 = Vec2::new(transform.translation.x, transform.translation.y);
                s.1 = direction.0;
            }
            direction.0 = MoveDirection::RightUp;
            return;
        }
        if key.pressed(KeyCode::Down) {
            if let Some(mut s) = segments.iter_mut().next() {
                s.0 = Vec2::new(transform.translation.x, transform.translation.y);
                s.1 = direction.0;
            }
            direction.0 = MoveDirection::RightDown;
            return;
        }

        if let Some(mut s) = segments.iter_mut().next() {
            s.0 = Vec2::new(transform.translation.x, transform.translation.y);
            s.1 = direction.0;
        }
        direction.0 = MoveDirection::Right;
    }

    if key.pressed(KeyCode::Up) {
        if let Some(mut s) = segments.iter_mut().next() {
            s.0 = Vec2::new(transform.translation.x, transform.translation.y);
            s.1 = direction.0;
        }
        direction.0 = MoveDirection::Up;
    }

    if key.pressed(KeyCode::Down) {
        if let Some(mut s) = segments.iter_mut().next() {
            s.0 = Vec2::new(transform.translation.x, transform.translation.y);
            s.1 = direction.0;
        }
        direction.0 = MoveDirection::Down;
    }
}
