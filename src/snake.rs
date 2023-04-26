use bevy::{
    prelude::{default, Color, Commands, Component, Transform, Vec3},
    sprite::{Sprite, SpriteBundle},
};

#[derive(Component)]
pub struct PlayerMovement;

pub fn setup_snake(mut commands: Commands) {
    commands.spawn((
        {
            SpriteBundle {
                sprite: Sprite {
                    color: Color::ANTIQUE_WHITE,
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(20.0, 20.0, 0.0),
                    ..default()
                },
                ..default()
            }
        },
        PlayerMovement,
    ));
}
