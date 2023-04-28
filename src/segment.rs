use crate::anguila::MoveDirection;
use bevy::{
    prelude::{default, Color, Commands, Resource, Transform, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
};

const SEGMENT_WIDTH: f32 = 12.0;
const SEGMENT_HEIGHT: f32 = 12.0;

pub struct Segment(pub Vec2, pub MoveDirection);

#[derive(Resource)]
pub struct Segments(pub Vec<Segment>);

pub fn add_segment(
    segments: &mut Vec<Segment>,
    commands: &mut Commands,
    position: Vec3,
    direction: MoveDirection,
) {
    segments.push(Segment(Vec2::new(position.x, position.y), direction));
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            ..default()
        },
        transform: Transform {
            scale: Vec3::new(SEGMENT_WIDTH, SEGMENT_HEIGHT, 0.0),
            translation: Vec3::new(position.x, position.y, 0.0),
            ..default()
        },
        ..default()
    });
}
