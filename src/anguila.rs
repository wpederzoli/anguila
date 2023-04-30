use bevy::{
    prelude::{default, Color, Commands, Component, Query, Transform, Vec2, Vec3, With},
    sprite::{Sprite, SpriteBundle},
};

use crate::segment::Segment;

pub const ANGUILA_WIDTH: f32 = 20.0;
pub const ANGUILA_HEIGHT: f32 = 20.0;
const ANGUILA_SPEED: f32 = 1.0;
const DIAGONAL_SPEED: f32 = ANGUILA_SPEED * 0.75;

#[derive(Clone, Copy, PartialEq)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    LeftUp,
    LeftDown,
    Right,
    RightUp,
    RightDown,
}

#[derive(Component)]
pub struct Anguila;

#[derive(Component)]
pub struct Direction(pub MoveDirection);

pub fn setup_anguila(mut commands: Commands) {
    commands.spawn((
        {
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.1, 0.7, 0.3, 0.5),
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(ANGUILA_WIDTH, ANGUILA_HEIGHT, 0.0),
                    ..default()
                },
                ..default()
            }
        },
        Anguila,
        Direction(MoveDirection::Up),
    ));
}

pub fn move_anguila(
    mut anguila: Query<(&mut Transform, &Direction), With<Anguila>>,
    mut segments: Query<&mut Segment>,
) {
    let (mut transform, direction) = anguila.single_mut();
    if let Some(mut segment) = segments.iter_mut().next() {
        if segment.1 == direction.0 {
            segment.0 = Vec2::new(transform.translation.x, transform.translation.y);
        }
    }
    move_towards(&mut transform.translation, &direction.0);
}

pub fn move_towards(translation: &mut Vec3, direction: &MoveDirection) {
    match direction {
        MoveDirection::Up => translation.y += ANGUILA_SPEED,
        MoveDirection::Down => translation.y -= ANGUILA_SPEED,
        MoveDirection::Left => translation.x -= ANGUILA_SPEED,
        MoveDirection::Right => translation.x += ANGUILA_SPEED,
        MoveDirection::LeftUp => {
            translation.x -= DIAGONAL_SPEED;
            translation.y += DIAGONAL_SPEED;
        }
        MoveDirection::LeftDown => {
            translation.x -= DIAGONAL_SPEED;
            translation.y -= DIAGONAL_SPEED;
        }
        MoveDirection::RightUp => {
            translation.x += DIAGONAL_SPEED;
            translation.y += DIAGONAL_SPEED;
        }
        MoveDirection::RightDown => {
            translation.x += DIAGONAL_SPEED;
            translation.y -= DIAGONAL_SPEED;
        }
    }
}
