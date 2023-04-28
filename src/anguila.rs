use bevy::{
    prelude::{default, Color, Commands, Component, Query, Transform, Vec3},
    sprite::{Sprite, SpriteBundle},
};

pub const ANGUILA_WIDTH: f32 = 20.0;
pub const ANGUILA_HEIGHT: f32 = 20.0;
const ANGUILA_SPEED: f32 = 1.0;
const DIAGONAL_SPEED: f32 = ANGUILA_SPEED * 0.75;

#[derive(Clone, Copy)]
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
                    color: Color::ANTIQUE_WHITE,
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

pub fn move_anguila(mut anguila: Query<(&mut Transform, &Direction, &Anguila)>) {
    for (mut transform, direction, _) in anguila.iter_mut() {
        match direction.0 {
            MoveDirection::Up => transform.translation.y += ANGUILA_SPEED,
            MoveDirection::Down => transform.translation.y -= ANGUILA_SPEED,
            MoveDirection::Left => transform.translation.x -= ANGUILA_SPEED,
            MoveDirection::Right => transform.translation.x += ANGUILA_SPEED,
            MoveDirection::LeftUp => {
                transform.translation.x -= DIAGONAL_SPEED;
                transform.translation.y += DIAGONAL_SPEED;
            }
            MoveDirection::LeftDown => {
                transform.translation.x -= DIAGONAL_SPEED;
                transform.translation.y -= DIAGONAL_SPEED;
            }
            MoveDirection::RightUp => {
                transform.translation.x += DIAGONAL_SPEED;
                transform.translation.y += DIAGONAL_SPEED;
            }
            MoveDirection::RightDown => {
                transform.translation.x += DIAGONAL_SPEED;
                transform.translation.y -= DIAGONAL_SPEED;
            }
        }
    }
}
