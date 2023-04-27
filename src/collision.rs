use bevy::{
    prelude::{Commands, Entity, Query, Transform, Vec3, With},
    sprite::SpriteBundle,
};

use crate::{
    anguila::{PlayerMovement, ANGUILA_HEIGHT, ANGUILA_WIDTH},
    targets::{Target, TARGET_HEIGHT, TARGET_WIDTH},
};

pub fn target_collision(
    mut commands: Commands,
    mut targets: Query<(Entity, &Transform, &Target)>,
    player: Query<&Transform, With<PlayerMovement>>,
) {
    let anguila = player.single();

    for (entity, position, _) in &mut targets {
        if is_colliding(&position.translation, &anguila.translation) {
            commands.entity(entity).remove::<SpriteBundle>();
            commands.entity(entity).remove::<Target>();
        }
    }
}

fn is_colliding(target: &Vec3, anguila: &Vec3) -> bool {
    if anguila.x + ANGUILA_WIDTH / 2.0 >= target.x - TARGET_WIDTH / 2.0
        && anguila.y + ANGUILA_HEIGHT / 2.0 >= target.y - TARGET_HEIGHT / 2.0
        && anguila.x - ANGUILA_WIDTH / 2.0 <= target.x + TARGET_WIDTH / 2.0
        && anguila.y - ANGUILA_HEIGHT / 2.0 <= target.y + TARGET_HEIGHT / 2.0
    {
        return true;
    }
    false
}
