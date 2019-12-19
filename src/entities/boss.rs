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
    let spawn_position = Vector3::new(

        constants::ARENA_MIN_X + (constants::ARENA_WIDTH / 2.0), constants::ARENA_MIN_Y + constants::ARENA_HEIGHT, constants::ENEMY_Z
    );
    let body_entity = enemy_pool[&"repeater_body".to_string()].clone();
    let body =  spawn_enemy(&entities, sprite_sheet, body_entity, spawn_position, &lazy_update);

    let repeater = Repeater {
        body
    };

    lazy_update.create_entity(entities).with(repeater).build();
}
