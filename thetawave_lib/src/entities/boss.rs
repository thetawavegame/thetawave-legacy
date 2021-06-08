use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{Entities, LazyUpdate, ReadExpect},
};

use crate::{
    components::RepeaterComponent,
    constants,
    entities::{EnemyType, MobType},
    resources::SpriteSheetsResource,
    spawnable::{entities::spawn_mob, resources::MobsResource},
};
use amethyst::prelude::Builder;

pub fn spawn_repeater(
    spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
    mobs_resource: &ReadExpect<MobsResource>,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let mut body_transform = Transform::default();
    body_transform.set_translation(Vector3::new(
        constants::ARENA_MIN_X + (constants::ARENA_WIDTH / 2.0),
        constants::ARENA_MIN_Y + constants::ARENA_HEIGHT + 100.0,
        constants::BOSS_Z_1,
    ));
    let mut head_transform = Transform::default();
    head_transform.set_translation(Vector3::new(
        constants::ARENA_MIN_X + (constants::ARENA_WIDTH / 2.0),
        constants::ARENA_MIN_Y + constants::ARENA_HEIGHT + 63.0,
        constants::BOSS_Z_2,
    ));
    let mut right_shoulder_transform = Transform::default();
    right_shoulder_transform.set_translation(Vector3::new(
        constants::ARENA_MIN_X + (constants::ARENA_WIDTH / 2.0) + 36.0,
        constants::ARENA_MIN_Y + constants::ARENA_HEIGHT + 98.0,
        constants::BOSS_Z_2,
    ));
    let mut left_shoulder_transform = Transform::default();
    left_shoulder_transform.set_translation(Vector3::new(
        constants::ARENA_MIN_X + (constants::ARENA_WIDTH / 2.0) - 36.0,
        constants::ARENA_MIN_Y + constants::ARENA_HEIGHT + 98.0,
        constants::BOSS_Z_2,
    ));

    let body = spawn_mob(
        &MobType::Enemy(EnemyType::RepeaterBody),
        body_transform,
        mobs_resource,
        spritesheets_resource,
        entities,
        lazy_update,
    );
    let head = spawn_mob(
        &MobType::Enemy(EnemyType::RepeaterHead),
        head_transform,
        mobs_resource,
        spritesheets_resource,
        entities,
        lazy_update,
    );
    let right_shoulder = spawn_mob(
        &MobType::Enemy(EnemyType::RepeaterRightShoulder),
        right_shoulder_transform,
        mobs_resource,
        spritesheets_resource,
        entities,
        lazy_update,
    );
    let left_shoulder = spawn_mob(
        &MobType::Enemy(EnemyType::RepeaterLeftShoulder),
        left_shoulder_transform,
        mobs_resource,
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
