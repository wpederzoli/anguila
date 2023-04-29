use bevy::{
    prelude::{Commands, Entity, Query, Transform, Vec3, With},
    sprite::SpriteBundle,
};

use crate::{
    anguila::{Anguila, Direction, ANGUILA_HEIGHT, ANGUILA_WIDTH},
    segment::{add_segment, Segment},
    targets::{Target, TARGET_HEIGHT, TARGET_WIDTH},
};

pub fn target_collision(
    mut commands: Commands,
    mut targets: Query<(Entity, &Transform, &Target)>,
    mut player: Query<(&Transform, &Direction), With<Anguila>>,
    mut segments: Query<&mut Segment>,
) {
    let (player_pos, player_dir) = player.single_mut();
    for (entity, target_pos, _) in &mut targets {
        if is_colliding(&target_pos.translation, &player_pos.translation) {
            commands.entity(entity).remove::<SpriteBundle>();
            commands.entity(entity).remove::<Target>();
            add_segment(&mut commands, &player_pos.translation, &player_dir.0);
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
