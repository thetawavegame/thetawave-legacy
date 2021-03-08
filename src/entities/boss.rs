use amethyst::{
    core::math::Vector3,
    ecs::prelude::{Entities, LazyUpdate, ReadExpect},
};

use crate::entities::spawn_enemy;
use crate::{
    components::RepeaterComponent,
    constants,
    entities::EnemyType,
    resources::{EnemiesResource, SpriteSheetsResource},
};
use amethyst::prelude::Builder;

pub fn spawn_repeater(
    spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
    enemies_resource: &ReadExpect<EnemiesResource>,
    entities: &Entities,
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

    let body = spawn_enemy(
        &EnemyType::RepeaterBody,
        body_position,
        enemies_resource,
        spritesheets_resource,
        entities,
        lazy_update,
    );
    let head = spawn_enemy(
        &EnemyType::RepeaterHead,
        head_position,
        enemies_resource,
        spritesheets_resource,
        entities,
        lazy_update,
    );
    let right_shoulder = spawn_enemy(
        &EnemyType::RepeaterRightShoulder,
        right_shoulder_position,
        enemies_resource,
        spritesheets_resource,
        entities,
        lazy_update,
    );
    let left_shoulder = spawn_enemy(
        &EnemyType::RepeaterLeftShoulder,
        left_shoulder_position,
        enemies_resource,
        spritesheets_resource,
        entities,
        lazy_update,
    );

    let repeater = RepeaterComponent {
        body,
        head,
        right_shoulder,
        left_shoulder,
    };

    lazy_update.create_entity(entities).with(repeater).build();
}
