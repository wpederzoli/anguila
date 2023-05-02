use crate::anguila::{
    get_next_destination, is_destination, move_towards, Anguila, Direction, MoveDirection,
    Position, ANGUILA_HEIGHT, ANGUILA_WIDTH,
};
use bevy::{
    prelude::{default, Color, Commands, Component, Query, Transform, Vec2, Vec3, With, Without},
    sprite::{Sprite, SpriteBundle},
};

#[derive(Component, Clone, Copy)]
pub struct Segment(pub Vec2, pub MoveDirection);

pub fn add_segment(
    commands: &mut Commands,
    position: &Vec3,
    direction: &MoveDirection,
    segments: &Query<(&Transform, &Direction), With<Segment>>,
) {
    let mut pos = *position;
    let mut dir = *direction;
    if let Some(segment) = segments.iter().last() {
        pos = segment.0.translation;
        dir = segment.1 .0;
    }

    let new_segment = Segment(Vec2::new(pos.x, pos.y), dir);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.1, 0.7, 0.3, 0.2),
                custom_size: Some(Vec2::new(0.5, 0.5)),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(ANGUILA_WIDTH, ANGUILA_HEIGHT, 0.0),
                translation: get_spawn_position(&pos, &dir),
                ..default()
            },
            ..default()
        },
        Direction(dir),
        new_segment,
    ));
}

pub fn move_segments(mut segments: Query<(&mut Transform, &mut Direction, &mut Segment)>) {
    let mut iter = segments.iter_mut();
    while let Some(mut segment) = iter.next() {
        if is_destination(&segment.0.translation, &segment.2 .0) {
            println!("reached destination: {}", segment.0.translation);
            let last_dest = segment.2 .0;
            let last_dir = segment.1 .0;
            segment.2 .0 = get_next_destination(&segment.2 .0, &segment.2 .1);
            segment.1 .0 = segment.2 .1;
            if let Some(mut s) = iter.next() {
                println!("set next dest: {:?}", last_dest);
                s.1 .0 = last_dir;
                s.2 .0 = last_dest;
                s.2 .1 = segment.1 .0;
            }
        }
    }

    for (mut transform, direction, _) in &mut segments {
        move_towards(&mut transform.translation, &direction.0);
    }
}

fn get_spawn_position(position: &Vec3, direction: &MoveDirection) -> Vec3 {
    match direction {
        MoveDirection::Up => Vec3::new(position.x, position.y - ANGUILA_HEIGHT, 0.0),
        MoveDirection::Down => Vec3::new(position.x, position.y + ANGUILA_HEIGHT, 0.0),
        MoveDirection::Left => Vec3::new(position.x + ANGUILA_WIDTH, position.y, 0.0),
        MoveDirection::Right => Vec3::new(position.x - ANGUILA_WIDTH, position.y, 0.0),
        MoveDirection::LeftUp => {
            Vec3::new(position.x + ANGUILA_WIDTH, position.y - ANGUILA_HEIGHT, 0.0)
        }
        MoveDirection::RightUp => {
            Vec3::new(position.x - ANGUILA_WIDTH, position.y - ANGUILA_HEIGHT, 0.0)
        }
        MoveDirection::LeftDown => {
            Vec3::new(position.x - ANGUILA_WIDTH, position.y + ANGUILA_HEIGHT, 0.0)
        }
        MoveDirection::RightDown => {
            Vec3::new(position.x + ANGUILA_WIDTH, position.y + ANGUILA_HEIGHT, 0.0)
        }
    }
}

fn get_next_position(position: &Vec3, direction: &MoveDirection) -> Vec3 {
    match *direction {
        MoveDirection::Up => Vec3::new(position.x, position.y + ANGUILA_HEIGHT, 0.0),
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
