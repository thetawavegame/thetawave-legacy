use crate::{
    entities::{ConsumableType, EffectType, EnemyType, ItemType},
    resources::{
        ConsumablesResource, EffectsResource, EnemiesResource, ItemsResource, SpriteSheetsResource,
    },
};
use amethyst::{
    core::{math::Vector3, transform::Transform, Parent},
    ecs::prelude::{Builder, Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{palette::Srgba, resources::Tint, SpriteRender, Transparent},
};

use rand::{thread_rng, Rng};

pub fn spawn_consumable(
    consumable_type: &ConsumableType,
    spawn_transform: Transform,
    consumables_resource: &ReadExpect<ConsumablesResource>,
    spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let consumable_data = consumables_resource.consumable_entities[consumable_type].clone();

    let sprite_render = SpriteRender {
        sprite_sheet: spritesheets_resource.spritesheets
            [&consumable_data.sprite_render_data.spritesheet]
            .clone(),
        sprite_number: consumable_data.sprite_render_data.initial_index,
    };

    lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(consumable_data.hitbox_component.clone())
        .with(consumable_data.consumable_component)
        .with(consumables_resource.motion2d_component.clone())
        .with(spawn_transform)
        .with(Transparent)
        .with(consumables_resource.despawn_border_component.clone())
        .build();
}

pub fn spawn_enemy(
    enemy_type: &EnemyType,
    spawn_transform: Transform,
    enemies_resource: &ReadExpect<EnemiesResource>,
    spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
) -> Entity {
    let enemy_data = enemies_resource[enemy_type].clone();

    let enemy_sprite_render = SpriteRender {
        sprite_sheet: spritesheets_resource.spritesheets
            [&enemy_data.sprite_render_data.spritesheet]
            .clone(),
        sprite_number: enemy_data.sprite_render_data.initial_index,
    };

    let enemy_entity = lazy_update
        .create_entity(entities)
        .with(enemy_sprite_render)
        .with(enemy_data.animation_component)
        .with(enemy_data.enemy_component)
        .with(enemy_data.hitbox_component)
        .with(enemy_data.motion2d_component)
        .with(enemy_data.health_component)
        .with(enemy_data.despawn_component)
        .with(spawn_transform)
        .with(Transparent)
        .build();

    if let Some(blaster_component) = enemy_data.blaster_component {
        lazy_update.insert(enemy_entity, blaster_component);
    }
    if let Some(autofire_component) = enemy_data.autofire_component {
        lazy_update.insert(enemy_entity, autofire_component);
    }
    if let Some(auto_child_entity_spawner_component) =
        enemy_data.auto_child_entity_spawner_component
    {
        lazy_update.insert(enemy_entity, auto_child_entity_spawner_component);
    }

    // Spawn thruster entity as child of enemy entity

    if let Some(thruster_data) = enemy_data.thruster_data {
        let thruster_parent = Parent::new(enemy_entity);

        let thruster_sprite_render = SpriteRender {
            sprite_sheet: spritesheets_resource.spritesheets["thrusters"].clone(),
            sprite_number: thruster_data.animation_component.start_idx,
        };

        let mut thruster_transform = Transform::default();
        thruster_transform.set_translation_y(thruster_data.y_offset);

        lazy_update
            .create_entity(entities)
            .with(thruster_parent)
            .with(thruster_transform)
            .with(thruster_sprite_render)
            .with(thruster_data.animation_component)
            .with(Transparent)
            .build();
    }

    enemy_entity
}

pub fn spawn_item(
    item_type: &ItemType,
    spawn_transform: Transform,
    items_resource: &ReadExpect<ItemsResource>,
    spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let item_data = items_resource.item_entities[item_type].clone();

    let sprite_render = SpriteRender {
        sprite_sheet: spritesheets_resource.spritesheets[&item_data.sprite_render_data.spritesheet]
            .clone(),
        sprite_number: item_data.sprite_render_data.initial_index,
    };

    let item_entity = lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(item_data.item_component)
        .with(items_resource.hitbox2d_component.clone())
        .with(items_resource.motion2d_component.clone())
        .with(spawn_transform)
        .with(Transparent)
        .with(items_resource.despawn_border_component.clone())
        .build();

    if let Some(animation_component) = item_data.animation_component {
        lazy_update.insert(item_entity, animation_component);
    }
}

pub fn spawn_effect(
    effect_type: &EffectType,
    spawn_transform: Transform,
    effects_resource: &ReadExpect<EffectsResource>,
    spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let effect_data = effects_resource[effect_type].clone();

    let sprite_render = SpriteRender {
        sprite_sheet: spritesheets_resource.spritesheets
            [&effect_data.sprite_render_data.spritesheet]
            .clone(),
        sprite_number: effect_data.sprite_render_data.initial_index,
    };

    let effect_entity = lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(spawn_transform)
        .with(Transparent)
        .build();

    if let Some(animation_component) = effect_data.animation_component {
        lazy_update.insert(effect_entity, animation_component);
    }

    if let Some(time_limit_component) = effect_data.time_limit_component {
        lazy_update.insert(effect_entity, time_limit_component);
    }

    if let Some(mut motion2d_component) = effect_data.motion2d_component.clone() {
        if let Some(random_initial_motion) = effect_data.random_initial_motion {
            motion2d_component.velocity.x = thread_rng().gen_range(
                random_initial_motion.linear.x.0,
                random_initial_motion.linear.x.1,
            );
            motion2d_component.velocity.y = thread_rng().gen_range(
                random_initial_motion.linear.y.0,
                random_initial_motion.linear.y.1,
            );
            motion2d_component.angular_velocity = thread_rng().gen_range(
                random_initial_motion.angular.0,
                random_initial_motion.angular.1,
            );
        }
        lazy_update.insert(effect_entity, motion2d_component);
    } else if effect_data.random_initial_motion.is_some() {
        panic!("Effects with RandomInitialMotion must also have a motion component.")
    }

    if let Some(fade_component) = effect_data.fade_component {
        let tint_component = Tint(Srgba::new(
            if let Some(red_change) = fade_component.red_change.clone() {
                red_change.value
            } else {
                1.0
            },
            if let Some(green_change) = fade_component.green_change.clone() {
                green_change.value
            } else {
                1.0
            },
            if let Some(blue_change) = fade_component.blue_change.clone() {
                blue_change.value
            } else {
                1.0
            },
            if let Some(alpha_change) = fade_component.alpha_change.clone() {
                alpha_change.value
            } else {
                1.0
            },
        ));

        lazy_update.insert(effect_entity, tint_component);
        lazy_update.insert(effect_entity, fade_component);
    }
}

pub fn spawn_giblets(
    enemy_type: &EnemyType,
    spawn_transform: Transform,
    effects_resource: &ReadExpect<EffectsResource>,
    spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    match enemy_type {
        EnemyType::Drone => {
            spawn_effect(
                &EffectType::DroneGiblet1,
                spawn_transform.clone(),
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );

            spawn_effect(
                &EffectType::DroneGiblet2,
                spawn_transform.clone(),
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );

            spawn_effect(
                &EffectType::DroneGiblet3,
                spawn_transform.clone(),
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );

            spawn_effect(
                &EffectType::DroneGiblet4,
                spawn_transform.clone(),
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );
        }

        EnemyType::Pawn => {
            spawn_effect(
                &EffectType::PawnGiblet1,
                spawn_transform.clone(),
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );

            spawn_effect(
                &EffectType::PawnGiblet2,
                spawn_transform.clone(),
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );

            spawn_effect(
                &EffectType::PawnGiblet3,
                spawn_transform.clone(),
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );

            spawn_effect(
                &EffectType::PawnGiblet4,
                spawn_transform.clone(),
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );
        }

        EnemyType::Strafer => {
            spawn_effect(
                &EffectType::StraferGiblet1,
                spawn_transform.clone(),
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );

            spawn_effect(
                &EffectType::StraferGiblet2,
                spawn_transform.clone(),
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );

            spawn_effect(
                &EffectType::StraferGiblet3,
                spawn_transform.clone(),
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );

            spawn_effect(
                &EffectType::StraferGiblet4,
                spawn_transform.clone(),
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );
        }

        EnemyType::Hauler => {
            spawn_effect(
                &EffectType::HaulerGiblet1,
                spawn_transform.clone(),
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );

            spawn_effect(
                &EffectType::HaulerGiblet2,
                spawn_transform.clone(),
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );

            spawn_effect(
                &EffectType::HaulerGiblet3,
                spawn_transform.clone(),
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );

            spawn_effect(
                &EffectType::HaulerGiblet4,
                spawn_transform.clone(),
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );
        }

        EnemyType::MissileLauncher => {
            spawn_effect(
                &EffectType::MissileLauncherGiblet1,
                spawn_transform.clone(),
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );

            spawn_effect(
                &EffectType::MissileLauncherGiblet2,
                spawn_transform.clone(),
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );

            spawn_effect(
                &EffectType::MissileLauncherGiblet3,
                spawn_transform.clone(),
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );

            spawn_effect(
                &EffectType::MissileLauncherGiblet4,
                spawn_transform.clone(),
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );
        }

        EnemyType::Missile => {
            spawn_effect(
                &EffectType::MissileGiblet1,
                spawn_transform.clone(),
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );

            spawn_effect(
                &EffectType::MissileGiblet2,
                spawn_transform.clone(),
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );

            spawn_effect(
                &EffectType::MissileGiblet3,
                spawn_transform.clone(),
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );

            spawn_effect(
                &EffectType::MissileGiblet4,
                spawn_transform.clone(),
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );
        }
        _ => {}
    }
}
