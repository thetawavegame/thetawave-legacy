use crate::resources::{EnemyEntityData, ThrusterEntityData};
use amethyst::{
    assets::Handle,
    core::{math::Vector3, transform::Transform, Named, Parent},
    ecs::prelude::{Builder, Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, SpriteSheet, Transparent},
};

pub fn spawn_enemy(
    entities: &Entities,
    sprite_sheet: Handle<SpriteSheet>,
    thruster_sprite_sheet: Option<Handle<SpriteSheet>>,
    enemy: EnemyEntityData,
    thruster: Option<ThrusterEntityData>,
    spawn_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) -> Entity {
    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: enemy.animation_component.start_idx,
    };

    let name = Named::new("enemy");

    let mut local_transform = Transform::default();
    local_transform.set_translation(spawn_position);

    let enemy_entity = lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(enemy.animation_component)
        .with(enemy.enemy_component)
        .with(enemy.hitbox_component)
        .with(enemy.motion2d_component)
        .with(enemy.health_component)
        .with(enemy.despawn_component)
        .with(local_transform)
        .with(Transparent)
        .with(name)
        .build();

    if let Some(blaster_component) = enemy.blaster_component {
        lazy_update.insert(enemy_entity, blaster_component);
    }
    if let Some(autofire_component) = enemy.autofire_component {
        lazy_update.insert(enemy_entity, autofire_component);
    }

    // Spawn thruster entity as child of enemy entity

    if let Some(thruster_data) = thruster {
        let thruster_parent = Parent::new(enemy_entity);

        let thruster_sprite_render = SpriteRender {
            sprite_sheet: thruster_sprite_sheet.unwrap(),
            sprite_number: thruster_data.animation_component.start_idx,
        };

        let mut thruster_local_transform = Transform::default();
        thruster_local_transform.set_translation_y(thruster_data.y_offset);

        lazy_update
            .create_entity(entities)
            .with(thruster_parent)
            .with(thruster_local_transform)
            .with(thruster_sprite_render)
            .with(thruster_data.animation_component)
            .with(Transparent)
            .build();
    }

    enemy_entity
}
