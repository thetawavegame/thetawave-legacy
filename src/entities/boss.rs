use amethyst::{
    ecs::prelude::{Entities, LazyUpdate, ReadExpect, Entity},
    renderer::{SpriteRender, SpriteSheet},
    core::math::Vector3,
    assets::Handle,
};

use crate::{
    components::{Enemy, Animation, Repeater},
    resources::{EnemyPool},
    constants,
};
use crate::entities::{spawn_animated_entity, spawn_enemy};
use amethyst::prelude::Builder;

pub fn spawn_repeater(entities: &Entities, sprite_sheet: Handle<SpriteSheet>, enemy_pool: &ReadExpect<EnemyPool>, lazy_update: &ReadExpect<LazyUpdate>) {
    let body_position = Vector3::new(

        constants::ARENA_MIN_X + (constants::ARENA_WIDTH / 2.0), constants::ARENA_MIN_Y + constants::ARENA_HEIGHT + 100.0, constants::BOSS_Z_1
    );
    let head_position = Vector3::new(

        constants::ARENA_MIN_X + (constants::ARENA_WIDTH / 2.0), constants::ARENA_MIN_Y + constants::ARENA_HEIGHT + 63.0, constants::BOSS_Z_2
    );
    let body_entity = enemy_pool[&"repeater_body".to_string()].clone();
    let head_entity = enemy_pool[&"repeater_head".to_string()].clone();
    let body =  spawn_enemy(&entities, sprite_sheet.clone(), body_entity, body_position, &lazy_update);
    let head =  spawn_enemy(&entities, sprite_sheet.clone(), head_entity, head_position, &lazy_update);

    let repeater = Repeater {
        body,
        head
    };

    lazy_update.create_entity(entities).with(repeater).build();
}
