use crate::{
    entities::EnemyType,
    resources::{
        ConsumableEntityData, ConsumablesResource, EnemiesResource, ItemEntityData, ItemsResource,
        SpriteSheetsResource,
    },
};
use amethyst::{
    core::{math::Vector3, transform::Transform, Named, Parent},
    ecs::prelude::{Builder, Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent},
};

pub fn spawn_consumable(
    sprite_resource: &ReadExpect<SpriteSheetsResource>,
    consumable_data: &ConsumableEntityData,
    consumables_resource: &ReadExpect<ConsumablesResource>,
    spawn_position: &Vector3<f32>,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_resource.spritesheets["consumables"].clone(),
        sprite_number: consumable_data.consumable_component.sprite_index,
    };

    let mut local_transform = Transform::default();
    local_transform.set_translation(*spawn_position);

    lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(consumable_data.hitbox_component.clone())
        .with(consumable_data.consumable_component.clone())
        .with(consumables_resource.motion2d_component.clone())
        .with(local_transform)
        .with(Transparent)
        .with(consumables_resource.despawn_border_component.clone())
        .build();
}

pub fn spawn_enemy(
    enemy_type: &EnemyType,
    sprite_sheets_resource: &ReadExpect<SpriteSheetsResource>,
    enemies_resource: &ReadExpect<EnemiesResource>,
    spawn_position: Vector3<f32>,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
) -> Entity {
    let enemy_data = enemies_resource[enemy_type].clone();

    //TODO: Consider moving all boss and enemy sprites onto single sprite sheet to remove this statement
    let sprite_sheets_key = match enemy_type {
        EnemyType::RepeaterBody
        | EnemyType::RepeaterHead
        | EnemyType::RepeaterLeftShoulder
        | EnemyType::RepeaterRightShoulder
        | EnemyType::RepeaterLeftArm
        | EnemyType::RepeaterRightArm => "repeater",
        _ => "enemies",
    };

    let enemy_sprite_render = SpriteRender {
        sprite_sheet: sprite_sheets_resource.spritesheets[sprite_sheets_key].clone(),
        sprite_number: enemy_data.animation_component.start_idx,
    };

    let mut enemy_transform = Transform::default();
    enemy_transform.set_translation(spawn_position);

    let enemy_entity = lazy_update
        .create_entity(entities)
        .with(enemy_sprite_render)
        .with(enemy_data.animation_component)
        .with(enemy_data.enemy_component)
        .with(enemy_data.hitbox_component)
        .with(enemy_data.motion2d_component)
        .with(enemy_data.health_component)
        .with(enemy_data.despawn_component)
        .with(enemy_transform)
        .with(Transparent)
        .build();

    if let Some(blaster_component) = enemy_data.blaster_component {
        lazy_update.insert(enemy_entity, blaster_component);
    }
    if let Some(autofire_component) = enemy_data.autofire_component {
        lazy_update.insert(enemy_entity, autofire_component);
    }
    if let Some(auto_child_enemy_spawner_component) = enemy_data.auto_child_enemy_spawner_component
    {
        lazy_update.insert(enemy_entity, auto_child_enemy_spawner_component);
    }

    // Spawn thruster entity as child of enemy entity

    if let Some(thruster_data) = enemy_data.thruster_data {
        let thruster_parent = Parent::new(enemy_entity);

        let thruster_sprite_render = SpriteRender {
            sprite_sheet: sprite_sheets_resource.spritesheets["thrusters"].clone(),
            sprite_number: thruster_data.animation_component.start_idx,
        };

        let mut thruster_transform = Transform::default();
        thruster_transform.set_translation_y(thruster_data.y_offset);

        lazy_update
            .create_entity(entities)
            .with(thruster_parent)
            .with(thruster_transform)
            .with(thruster_sprite_render)
            .with(thruster_data.animation_component.clone())
            .with(Transparent)
            .build();
    }

    enemy_entity
}

pub fn spawn_item(
    spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
    item: ItemEntityData,
    items_resource: &ReadExpect<ItemsResource>,
    spawn_position: Vector3<f32>,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let sprite_render = SpriteRender {
        sprite_sheet: spritesheets_resource.spritesheets["items"].clone(),
        sprite_number: item.item_component.sprite_index,
    };
    let mut local_transform = Transform::default();
    local_transform.set_translation(spawn_position);

    let name = Named::new("item");

    println!("{} spawned!", item.item_component.name);

    let item_entity = lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(item.item_component)
        .with(items_resource.hitbox2d_component.clone())
        .with(items_resource.motion2d_component.clone())
        .with(local_transform)
        .with(Transparent)
        .with(name)
        .with(items_resource.despawn_border_component.clone())
        .build();

    if let Some(animation_component) = item.animation_component {
        lazy_update.insert(item_entity, animation_component);
    }
}
