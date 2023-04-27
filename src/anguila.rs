use bevy::{
    prelude::{default, Color, Commands, Component, Entity, Query, Transform, Vec3, With},
    sprite::{Sprite, SpriteBundle},
};

use crate::targets::{Target, TARGET_HEIGHT, TARGET_WIDTH};

pub const ANGUILA_WIDTH: f32 = 20.0;
pub const ANGUILA_HEIGHT: f32 = 20.0;

#[derive(Component)]
pub struct PlayerMovement;

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
        PlayerMovement,
    ));
}

pub fn target_collision(
    mut commands: Commands,
    mut targets: Query<(Entity, &Transform, &Target)>,
    player: Query<&Transform, With<PlayerMovement>>,
) {
    let anguila = player.single();

    for (entity, position, _) in &mut targets {
        if is_colliding(position.translation, anguila.translation) {
            commands.entity(entity).remove::<SpriteBundle>();
            commands.entity(entity).remove::<Target>();
        }
    }
}

fn is_colliding(target: Vec3, anguila: Vec3) -> bool {
    if anguila.x + ANGUILA_WIDTH / 2.0 >= target.x - TARGET_WIDTH / 2.0
        && anguila.y + ANGUILA_HEIGHT / 2.0 >= target.y - TARGET_HEIGHT / 2.0
        && anguila.x - ANGUILA_WIDTH / 2.0 <= target.x + TARGET_WIDTH / 2.0
        && anguila.y - ANGUILA_HEIGHT / 2.0 <= target.y + TARGET_HEIGHT / 2.0
    {
        return true;
    }
    false
}
