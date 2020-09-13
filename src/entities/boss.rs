use amethyst::{
    assets::Handle,
    core::math::Vector3,
    ecs::prelude::{Entities, LazyUpdate, ReadExpect},
    renderer::SpriteSheet,
};

use crate::entities::spawn_enemy;
use crate::{components::Repeater, constants, resources::EnemyPool};
use amethyst::prelude::Builder;

pub fn spawn_repeater(
    entities: &Entities,
    sprite_sheet: Handle<SpriteSheet>,
    enemy_pool: &ReadExpect<EnemyPool>,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let body_position = Vector3::new(
        constants::ARENA_MIN_X + (constants::ARENA_WIDTH / 2.0),
        constants::ARENA_MIN_Y + constants::ARENA_HEIGHT + 100.0,
        constants::BOSS_Z_1,
    );
    let head_position = Vector3::new(
        constants::ARENA_MIN_X + (constants::ARENA_WIDTH / 2.0),
        constants::ARENA_MIN_Y + constants::ARENA_HEIGHT + 63.0,
        constants::BOSS_Z_2,
    );
    let right_shoulder_position = Vector3::new(
        constants::ARENA_MIN_X + (constants::ARENA_WIDTH / 2.0) + 36.0,
        constants::ARENA_MIN_Y + constants::ARENA_HEIGHT + 98.0,
        constants::BOSS_Z_1,
    );
    let left_shoulder_position = Vector3::new(
        constants::ARENA_MIN_X + (constants::ARENA_WIDTH / 2.0) - 36.0,
        constants::ARENA_MIN_Y + constants::ARENA_HEIGHT + 98.0,
        constants::BOSS_Z_1,
    );
    let _right_arm_position = Vector3::new(
        constants::ARENA_MIN_X + (constants::ARENA_WIDTH / 2.0) + 30.0,
        constants::ARENA_MIN_Y + constants::ARENA_HEIGHT + 40.0,
        constants::BOSS_Z_2,
    );
    let body_entity_data = enemy_pool[&"repeater_body".to_string()].clone();
    let head_entity_data = enemy_pool[&"repeater_head".to_string()].clone();
    let right_shoulder_entity_data = enemy_pool[&"repeater_right_shoulder".to_string()].clone();
    let left_shoulder_entity_data = enemy_pool[&"repeater_left_shoulder".to_string()].clone();
    let _right_arm_entity_data = enemy_pool[&"repeater_right_arm".to_string()].clone();

    let body = spawn_enemy(
        &entities,
        sprite_sheet.clone(),
        body_entity_data,
        body_position,
        &lazy_update,
    );
    let head = spawn_enemy(
        &entities,
        sprite_sheet.clone(),
        head_entity_data,
        head_position,
        &lazy_update,
    );
    let right_shoulder = spawn_enemy(
        &entities,
        sprite_sheet.clone(),
        right_shoulder_entity_data,
        right_shoulder_position,
        &lazy_update,
    );
    let left_shoulder = spawn_enemy(
        &entities,
        sprite_sheet.clone(),
        left_shoulder_entity_data,
        left_shoulder_position,
        &lazy_update,
    );
    let repeater = Repeater {
        body,
        head,
        right_shoulder,
        left_shoulder,
    };

    lazy_update.create_entity(entities).with(repeater).build();
}
