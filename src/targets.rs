use bevy::{
    prelude::{default, Color, Commands, Transform, Vec3},
    sprite::{Sprite, SpriteBundle},
};
use rand::Rng;

pub const SPAWN_TIME: f32 = 3.0;

pub fn spawn_targets(mut commands: Commands) {
    let spawn = rand::thread_rng().gen_range(0..2);

    if spawn > 0 {
        let spawn_x = rand::thread_rng().gen_range(-500..500) as f32; // TODO: account for window width
        let spawn_y = rand::thread_rng().gen_range(-500..500) as f32; // TODO: account for window height

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::SEA_GREEN,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(5.0, 5.0, 0.0),
                translation: Vec3::new(spawn_x, spawn_y, 0.0),
                ..default()
            },
            ..default()
        });
    }
}
