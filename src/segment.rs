use crate::anguila::{Anguila, Direction, MoveDirection, ANGUILA_HEIGHT, ANGUILA_WIDTH};
use bevy::{
    prelude::{
        default, Color, Commands, Component, Query, Resource, Transform, Vec2, Vec3, With, Without,
    },
    sprite::{Sprite, SpriteBundle},
};

const SEGMENT_WIDTH: f32 = 12.0;
const SEGMENT_HEIGHT: f32 = 12.0;

#[derive(Component, Clone, Copy)]
pub struct Segment(pub Vec2, pub MoveDirection);

#[derive(Resource)]
pub struct Segments(pub Vec<Segment>);

pub fn add_segment(
    segments: &mut Vec<Segment>,
    commands: &mut Commands,
    position: &Vec3,
    direction: &MoveDirection,
) {
    let mut pos = *position;
    let mut dir = *direction;

    if let Some(last_segment) = segments.last() {
        pos = Vec3::new(last_segment.0.x, last_segment.0.y, 0.0);
        dir = last_segment.1
    }

    let new_segment = get_new_segment(&pos, &dir);

    segments.push(new_segment);
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
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
        MoveDirection::Up => {
            return Segment(
                Vec2::new(position.x, position.y - ANGUILA_HEIGHT),
                *direction,
            );
        }
        _ => Segment(Vec2::new(position.x, position.y), *direction),
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
