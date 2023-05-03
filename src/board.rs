use bevy::{
    prelude::{Color, Commands, Transform, Vec3},
    sprite::{Sprite, SpriteBundle},
    utils::default,
};
use rand::Rng;

use crate::anguila::{MoveDirection, ANGUILA_WIDTH};

pub const CELL_SIZE: f32 = ANGUILA_WIDTH;
pub const BOARD_WIDTH: i32 = 20;
pub const BOARD_HEIGHT: i32 = 20;

pub fn init_board(mut commands: Commands) {
    for row in 0..BOARD_WIDTH {
        for col in 0..BOARD_HEIGHT {
            let x = col as f32 * CELL_SIZE;
            let y = row as f32 * CELL_SIZE;

            let rand_color = rand::thread_rng().gen_range(0.0..1.0);
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::Rgba {
                        red: 0.0,
                        green: 0.0,
                        blue: rand_color,
                        alpha: 0.75,
                    },
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(CELL_SIZE, CELL_SIZE, 0.0),
                    translation: Vec3::new(x, y, 0.0),
                    ..default()
                },
                ..default()
            });
        }
    }
}

pub fn is_border_collision(position: &Vec3, direction: &MoveDirection) -> bool {
    match *direction {
        MoveDirection::Up => {
            if position.y + ANGUILA_WIDTH > BOARD_HEIGHT as f32 * CELL_SIZE {
                return true;
            }

            return false;
        }
        MoveDirection::Down => {
            if position.y - ANGUILA_WIDTH < 0.0 - CELL_SIZE {
                return true;
            } else {
                return false;
            }
        }
        MoveDirection::Left => {
            if position.x - ANGUILA_WIDTH < 0.0 - CELL_SIZE {
                return true;
            } else {
                return false;
            }
        }
        MoveDirection::Right => {
            if position.x + ANGUILA_WIDTH > BOARD_WIDTH as f32 * CELL_SIZE {
                return true;
            } else {
                return false;
            }
        }
        MoveDirection::LeftUp => {
            if position.x - ANGUILA_WIDTH < 0.0 - CELL_SIZE
                || position.y + ANGUILA_WIDTH > BOARD_HEIGHT as f32 * CELL_SIZE
            {
                return true;
            } else {
                return false;
            }
        }
        MoveDirection::RightUp => {
            if position.x + ANGUILA_WIDTH > BOARD_WIDTH as f32 * CELL_SIZE
                || position.y + ANGUILA_WIDTH > BOARD_HEIGHT as f32 * CELL_SIZE
            {
                return true;
            } else {
                return false;
            }
        }
        MoveDirection::LeftDown => {
            if position.x - ANGUILA_WIDTH < 0.0 - CELL_SIZE
                || position.y - ANGUILA_WIDTH < 0.0 - CELL_SIZE
            {
                return true;
            } else {
                return false;
            }
        }
        MoveDirection::RightDown => {
            if position.x + ANGUILA_WIDTH > BOARD_WIDTH as f32 * CELL_SIZE
                || position.y - ANGUILA_WIDTH < 0.0 - CELL_SIZE
            {
                return true;
            } else {
                return false;
            }
        }
    }
}
