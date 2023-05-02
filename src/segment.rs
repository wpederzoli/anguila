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

pub fn add_segment(commands: &mut Commands, position: &Vec3, direction: &MoveDirection) {
    let new_segment = Segment(Vec2::new(position.x, position.y), *direction);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.1, 0.7, 0.3, 0.2),
                custom_size: Some(Vec2::new(0.5, 0.5)),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(ANGUILA_WIDTH, ANGUILA_HEIGHT, 0.0),
                translation: get_spawn_position(&position, &direction),
                ..default()
            },
            ..default()
        },
        Direction(*direction),
        new_segment,
    ));
}

pub fn move_segments(mut segments: Query<(&mut Transform, &mut Direction, &mut Segment)>) {
    let mut iter = segments.iter_mut();
    if let Some(mut segment) = iter.next() {
        let mut current_segment = segment;
        while let Some(mut next_segment) = iter.next() {
            if is_destination(&current_segment.0.translation, &current_segment.2 .0) {
                println!("reached destination");
                next_segment.2 .0 = current_segment.2 .0;
                next_segment.2 .1 = current_segment.2 .1;
                current_segment.2 .0 = get_next_destination(
                    &Vec2::new(
                        current_segment.0.translation.x,
                        current_segment.0.translation.y,
                    ),
                    &current_segment.2 .1,
                );
                current_segment.1 .0 = current_segment.2 .1;
            }
            move_towards(&mut current_segment.0.translation, &current_segment.1 .0);
            current_segment = next_segment;
        }

        if is_destination(&current_segment.0.translation, &current_segment.2 .0) {
            println!("reached destination no more segments");
        }
        move_towards(&mut current_segment.0.translation, &current_segment.1 .0);
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
