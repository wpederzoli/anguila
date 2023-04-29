use crate::anguila::{Anguila, Direction, MoveDirection, ANGUILA_HEIGHT, ANGUILA_WIDTH};
use bevy::{
    prelude::{default, Color, Commands, Component, Query, Transform, Vec2, Vec3, With, Without},
    sprite::{Sprite, SpriteBundle},
};

const SEGMENT_WIDTH: f32 = 12.0;
const SEGMENT_HEIGHT: f32 = 12.0;

#[derive(Component, Clone, Copy)]
pub struct Segment(pub Vec2, pub MoveDirection);

pub fn add_segment(commands: &mut Commands, position: &Vec3, direction: &MoveDirection) {
    let new_segment = get_new_segment(&position, &direction);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.1, 0.7, 0.3, 0.2),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(SEGMENT_WIDTH, SEGMENT_HEIGHT, 0.0),
                translation: Vec3::new(new_segment.0.x, new_segment.0.y, 0.0),
                ..default()
            },
            ..default()
        },
        new_segment,
    ));
}

pub fn move_segments(
    mut segments: Query<(&mut Transform, &mut Segment), Without<Anguila>>,
    player: Query<(&Transform, &Direction), With<Anguila>>,
) {
    let player = player.single();
    let mut player_pos = player.0.translation;
    let mut player_dir = player.1 .0;

    for (mut seg_pos, mut segment) in &mut segments {
        let next_pos = get_next_position(&player_pos, &player_dir);
        let next_dir = segment.1;

        seg_pos.translation = next_pos;
        segment.1 = player_dir;

        player_pos = next_pos;
        player_dir = next_dir;
    }
}

fn get_new_segment(position: &Vec3, direction: &MoveDirection) -> Segment {
    match direction {
        MoveDirection::Up => Segment(
            Vec2::new(position.x, position.y - ANGUILA_HEIGHT),
            *direction,
        ),
        MoveDirection::Down => Segment(
            Vec2::new(position.x, position.y + ANGUILA_HEIGHT),
            *direction,
        ),
        MoveDirection::Left => Segment(
            Vec2::new(position.x + ANGUILA_WIDTH, position.y),
            *direction,
        ),
        MoveDirection::Right => Segment(
            Vec2::new(position.x - ANGUILA_WIDTH, position.y),
            *direction,
        ),
        MoveDirection::LeftUp => Segment(
            Vec2::new(position.x + ANGUILA_WIDTH, position.y - ANGUILA_HEIGHT),
            *direction,
        ),
        MoveDirection::RightUp => Segment(
            Vec2::new(position.x - ANGUILA_WIDTH, position.y - ANGUILA_HEIGHT),
            *direction,
        ),
        MoveDirection::LeftDown => Segment(
            Vec2::new(position.x - ANGUILA_WIDTH, position.y + ANGUILA_HEIGHT),
            *direction,
        ),
        MoveDirection::RightDown => Segment(
            Vec2::new(position.x + ANGUILA_WIDTH, position.y + ANGUILA_HEIGHT),
            *direction,
        ),
    }
}

fn get_next_position(position: &Vec3, direction: &MoveDirection) -> Vec3 {
    match *direction {
        MoveDirection::Up => Vec3::new(position.x, position.y - ANGUILA_HEIGHT, 0.0),
        MoveDirection::Down => Vec3::new(position.x, position.y + ANGUILA_HEIGHT, 0.0),
        MoveDirection::Left => Vec3::new(position.x + ANGUILA_WIDTH, position.y, 0.0),
        MoveDirection::LeftUp => {
            Vec3::new(position.x + ANGUILA_WIDTH, position.y - ANGUILA_HEIGHT, 0.0)
        }
        MoveDirection::LeftDown => {
            Vec3::new(position.x + ANGUILA_WIDTH, position.y + ANGUILA_HEIGHT, 0.0)
        }
        MoveDirection::Right => Vec3::new(position.x - ANGUILA_WIDTH, position.y, 0.0),
        MoveDirection::RightUp => {
            Vec3::new(position.x - ANGUILA_WIDTH, position.y - ANGUILA_HEIGHT, 0.0)
        }
        MoveDirection::RightDown => {
            Vec3::new(position.x - ANGUILA_WIDTH, position.y + ANGUILA_HEIGHT, 0.0)
        }
    }
}
