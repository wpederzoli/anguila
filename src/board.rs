use bevy::{
    prelude::{Color, Commands, Transform, Vec3},
    sprite::{Sprite, SpriteBundle},
    utils::default,
};

use crate::anguila::{MoveDirection, ANGUILA_WIDTH};

pub const CELL_SIZE: f32 = ANGUILA_WIDTH;
pub const BOARD_WIDTH: i32 = 20;
pub const BOARD_HEIGHT: i32 = 20;

pub fn init_board(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(0.2, 0.3, 0.6, 0.0),
            ..default()
        },
        transform: Transform {
            scale: Vec3::new(
                BOARD_WIDTH as f32 * CELL_SIZE,
                BOARD_HEIGHT as f32 * CELL_SIZE,
                0.0,
            ),
            translation: Vec3::new((-BOARD_WIDTH / 2) as f32, (-BOARD_HEIGHT / 2) as f32, 0.0),
            ..default()
        },
        ..default()
    });
}

pub fn is_border_collision(position: &Vec3, direction: &MoveDirection) -> bool {
    match *direction {
        MoveDirection::Up => {
            if position.y + ANGUILA_WIDTH > BOARD_HEIGHT as f32 / 2.0 * CELL_SIZE {
                return true;
            }

            return false;
        }
        MoveDirection::Down => {
            if position.y - ANGUILA_WIDTH < -BOARD_HEIGHT as f32 / 2.0 * CELL_SIZE - CELL_SIZE {
                return true;
            } else {
                return false;
            }
        }
        MoveDirection::Left => {
            if position.x - ANGUILA_WIDTH < (-BOARD_WIDTH as f32 / 2.0 * CELL_SIZE) - CELL_SIZE {
                return true;
            } else {
                return false;
            }
        }
        MoveDirection::Right => {
            if position.x + ANGUILA_WIDTH > BOARD_WIDTH as f32 / 2.0 * CELL_SIZE {
                return true;
            } else {
                return false;
            }
        }
    }
}
