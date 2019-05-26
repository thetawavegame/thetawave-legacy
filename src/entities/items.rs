use amethyst::{
    ecs::prelude::{Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent},
    core::{
        transform::Transform,
        nalgebra::Vector3,
    }
};

use crate::{
    components::Item,
    resources::SpriteResource,
};

use std::collections::HashMap;


const ITEM_HEIGHT: f32 = 14.0;
pub const ITEM_WIDTH: f32 = 14.0;
const ITEM_HITBOX_WIDTH: f32 = 4.0;
const ITEM_HITBOX_HEIGHT: f32 = 4.0;


pub fn spawn_item(entities: &Entities, item_resource: &ReadExpect<SpriteResource>, sprite_number: usize, stat_effects: HashMap<&'static str, f32>,spawn_position: Vector3<f32>, lazy_update: &ReadExpect<LazyUpdate>) {
    let item_entity: Entity = entities.create();

    let mut local_transform = Transform::default();
    local_transform.set_position(spawn_position);

    let sprite_render = SpriteRender {
        sprite_sheet: item_resource.sprite_sheet.clone(),
        sprite_number: sprite_number,
    };

    lazy_update.insert(item_entity, sprite_render);
    lazy_update.insert(item_entity, Item {
        height: ITEM_HEIGHT,
        width: ITEM_WIDTH,
        hitbox_height: ITEM_HITBOX_HEIGHT,
        hitbox_width: ITEM_HITBOX_WIDTH,
        stat_effects: stat_effects
    });
    lazy_update.insert(item_entity, local_transform);
    lazy_update.insert(item_entity, Transparent);

}