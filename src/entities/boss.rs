use amethyst::{
    assets::Handle,
    core::math::Vector3,
    ecs::prelude::{Entities, LazyUpdate, ReadExpect},
    renderer::SpriteSheet,
};

use crate::entities::spawn_enemy;
use crate::{
    components::RepeaterComponent, constants, entities::EntityType, resources::EnemiesResource,
};
use amethyst::prelude::Builder;

pub fn spawn_repeater(
    entities: &Entities,
    sprite_sheet: Handle<SpriteSheet>,
    enemy_pool: &ReadExpect<EnemiesResource>,
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
    let body_entity_data = enemy_pool[&EntityType::RepeaterBody].clone();
    let head_entity_data = enemy_pool[&EntityType::RepeaterHead].clone();
    let right_shoulder_entity_data = enemy_pool[&EntityType::RepeaterRightShoulder].clone();
    let left_shoulder_entity_data = enemy_pool[&EntityType::RepeaterLeftShoulder].clone();
    let _right_arm_entity_data = enemy_pool[&EntityType::RepeaterRightArm].clone();

    let body = spawn_enemy(
        &entities,
        sprite_sheet.clone(),
        None,
        body_entity_data,
        None,
        body_position,
        &lazy_update,
    );
    let head = spawn_enemy(
        &entities,
        sprite_sheet.clone(),
        None,
        head_entity_data,
        None,
        head_position,
        &lazy_update,
    );
    let right_shoulder = spawn_enemy(
        &entities,
        sprite_sheet.clone(),
        None,
        right_shoulder_entity_data,
        None,
        right_shoulder_position,
        &lazy_update,
    );
    let left_shoulder = spawn_enemy(
        &entities,
        sprite_sheet.clone(),
        None,
        left_shoulder_entity_data,
        None,
        left_shoulder_position,
        &lazy_update,
    );
    let repeater = RepeaterComponent {
        body,
        head,
        right_shoulder,
        left_shoulder,
    };

    lazy_update.create_entity(entities).with(repeater).build();
}
