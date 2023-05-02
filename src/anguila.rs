use bevy::{
    prelude::{default, Color, Commands, Component, Query, Transform, Vec2, Vec3, With},
    sprite::{Sprite, SpriteBundle},
};

use crate::{
    board::{is_border_collision, CELL_SIZE},
    segment::Segment,
};

pub const ANGUILA_WIDTH: f32 = 20.0;
pub const ANGUILA_HEIGHT: f32 = 20.0;
const ANGUILA_SPEED: f32 = 1.;
const DIAGONAL_SPEED: f32 = ANGUILA_SPEED * 0.75;

//TODO: Remove diagonal movement for v1
#[derive(Clone, Copy, PartialEq, Debug)]
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

#[derive(Component)]
pub struct Position(pub Vec2, pub MoveDirection);

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
        Position(Vec2::new(0.0, 20.0), MoveDirection::Up),
        Direction(MoveDirection::Up),
    ));
}

pub fn move_anguila(
    mut anguila: Query<(&mut Transform, &mut Direction, &mut Position), With<Anguila>>,
    mut segment: Query<&mut Segment>,
) {
    let (mut transform, mut direction, mut position) = anguila.single_mut();
    if !is_border_collision(&transform.translation, &direction.0) {
        move_towards(&mut transform.translation, &direction.0);
        if is_destination(&transform.translation, &position.0) {
            position.0 = get_next_destination(&position.0, &position.1);
            direction.0 = position.1;
        }
    }
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

pub fn is_destination(position: &Vec3, destination: &Vec2) -> bool {
    position.x == destination.x && position.y == destination.y
}

pub fn get_next_destination(current: &Vec2, direction: &MoveDirection) -> Vec2 {
    match *direction {
        MoveDirection::Up => Vec2::new(current.x, current.y + CELL_SIZE),
        MoveDirection::Down => Vec2::new(current.x, current.y - CELL_SIZE),
        MoveDirection::Left => Vec2::new(current.x - CELL_SIZE, current.y),
        MoveDirection::Right => Vec2::new(current.x + CELL_SIZE, current.y),
        MoveDirection::LeftUp => Vec2::new(current.x - CELL_SIZE, current.y + CELL_SIZE),
        MoveDirection::RightUp => Vec2::new(current.x + CELL_SIZE, current.y + CELL_SIZE),
        MoveDirection::LeftDown => Vec2::new(current.x - CELL_SIZE, current.y - CELL_SIZE),
        MoveDirection::RightDown => Vec2::new(current.x + CELL_SIZE, current.y - CELL_SIZE),
    }
}
