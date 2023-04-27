use bevy::{
    prelude::{default, Color, Commands, Component, Query, Transform, Vec3},
    sprite::{Sprite, SpriteBundle},
};

pub const ANGUILA_WIDTH: f32 = 20.0;
pub const ANGUILA_HEIGHT: f32 = 20.0;
const ANGUILA_SPEED: f32 = 1.0;
const DIAGONAL_SPEED: f32 = ANGUILA_SPEED * 0.75;

pub enum Direction {
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
pub struct PlayerMovement {
    pub direction: Direction,
}

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
        PlayerMovement {
            direction: Direction::Up,
        },
    ));
}

pub fn move_anguila(mut anguila: Query<(&mut Transform, &mut PlayerMovement)>) {
    for (mut transform, player) in anguila.iter_mut() {
        match player.direction {
            Direction::Up => transform.translation.y += ANGUILA_SPEED,
            Direction::Down => transform.translation.y -= ANGUILA_SPEED,
            Direction::Left => transform.translation.x -= ANGUILA_SPEED,
            Direction::Right => transform.translation.x += ANGUILA_SPEED,
            Direction::LeftUp => {
                transform.translation.x -= DIAGONAL_SPEED;
                transform.translation.y += DIAGONAL_SPEED;
            }
            Direction::LeftDown => {
                transform.translation.x -= DIAGONAL_SPEED;
                transform.translation.y -= DIAGONAL_SPEED;
            }
            Direction::RightUp => {
                transform.translation.x += DIAGONAL_SPEED;
                transform.translation.y += DIAGONAL_SPEED;
            }
            Direction::RightDown => {
                transform.translation.x += DIAGONAL_SPEED;
                transform.translation.y -= DIAGONAL_SPEED;
            }
        }
    }
}
