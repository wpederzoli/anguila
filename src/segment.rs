use crate::anguila::{Anguila, Direction, MoveDirection};
use bevy::{
    prelude::{
        default, Color, Commands, Component, Query, ResMut, Resource, Transform, Vec2, Vec3,
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

    let new_segment = Segment(Vec2::new(pos.x, pos.y), dir);

    segments.push(new_segment);
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(SEGMENT_WIDTH, SEGMENT_HEIGHT, 0.0),
                translation: pos,
                ..default()
            },
            ..default()
        },
        new_segment,
    ));
}

pub fn update_segments(
    mut segments: ResMut<Segments>,
    player: Query<(&Transform, &Anguila, &Direction)>,
) {
    if segments.0.len() > 0 {
        for i in 0..segments.0.len() - 1 {
            let index = segments.0.len() - i;
            let direction = segments.0[index + 1].1;
            let position = Vec2::new(segments.0[index + 1].0.x, segments.0[index + 1].0.y);
            std::mem::replace(&mut segments.0[index], Segment(position, direction));
        }

        let anguila = player.single();
        std::mem::replace(
            &mut segments.0[0],
            Segment(
                Vec2::new(anguila.0.translation.x, anguila.0.translation.y),
                anguila.2 .0,
            ),
        );
    }
}

pub fn move_segments(mut segments: Query<(&mut Transform, &Segment)>) {
    for (mut transform, seg) in &mut segments {
        transform.translation = Vec3::new(seg.0.x, seg.0.y, 0.0);
    }
}
