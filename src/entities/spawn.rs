use crate::{
    entities::{ConsumableType, EffectType, ItemType, MobType, SpawnableType},
    resources::{
        ConsumablesResource, DropProbabilities, DropRolls, DropTableType, DropTablesResource,
        EffectsResource, ItemsResource, MobsResource, RollProbabilities, SpriteSheetsResource,
    },
};
use amethyst::{
    core::{transform::Transform, Parent},
    ecs::prelude::{Builder, Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{palette::Srgba, resources::Tint, SpriteRender, Transparent},
};

use rand::{thread_rng, Rng};

pub type LootTable = Vec<(Option<SpawnableType>, f32)>;

pub fn choose_loot(drop_probs: &DropProbabilities) -> &SpawnableType {
    let prob_space = drop_probs
        .iter()
        .fold(0.0, |sum, drop_prob| sum + drop_prob.1);

    let pos = rand::thread_rng().gen::<f32>() * prob_space;
    let mut sum = 0.0;
    for drop_prob in drop_probs.iter() {
        sum += drop_prob.1;
        if sum > pos {
            return &drop_prob.0;
        }
    }
    unreachable!("Error in probabilities of random spawnable pool.")
}

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

pub fn spawn_consumable_drop(
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

    let mut motion2d_component = consumables_resource.motion2d_component.clone();
    motion2d_component.velocity.x = thread_rng().gen_range(
        consumables_resource.random_initial_motion.linear.x.0,
        consumables_resource.random_initial_motion.linear.x.1,
    );
    motion2d_component.velocity.y = thread_rng().gen_range(
        consumables_resource.random_initial_motion.linear.y.0,
        consumables_resource.random_initial_motion.linear.y.1,
    );
    motion2d_component.angular_velocity = thread_rng().gen_range(
        consumables_resource.random_initial_motion.angular.0,
        consumables_resource.random_initial_motion.angular.1,
    );

    lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(consumable_data.hitbox_component.clone())
        .with(consumable_data.consumable_component)
        .with(motion2d_component)
        .with(spawn_transform)
        .with(Transparent)
        .with(consumables_resource.despawn_border_component.clone())
        .build();
}

pub fn spawn_mob(
    mob_type: &MobType,
    spawn_transform: Transform,
    mobs_resource: &ReadExpect<MobsResource>,
    spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
) -> Entity {
    let mob_data = mobs_resource[mob_type].clone();

    let mob_sprite_render = SpriteRender {
        sprite_sheet: spritesheets_resource.spritesheets[&mob_data.sprite_render_data.spritesheet]
            .clone(),
        sprite_number: mob_data.sprite_render_data.initial_index,
    };

    let mob_entity = lazy_update
        .create_entity(entities)
        .with(mob_sprite_render)
        .with(mob_data.animation_component)
        .with(mob_data.mob_component)
        .with(mob_data.hitbox_component)
        .with(mob_data.motion2d_component)
        .with(mob_data.health_component)
        .with(mob_data.despawn_component)
        .with(spawn_transform)
        .with(Transparent)
        .build();

    if let Some(blaster_component) = mob_data.blaster_component {
        lazy_update.insert(mob_entity, blaster_component);
    }
    if let Some(autofire_component) = mob_data.autofire_component {
        lazy_update.insert(mob_entity, autofire_component);
    }
    if let Some(auto_child_entity_spawner_component) = mob_data.auto_spawner_component {
        lazy_update.insert(mob_entity, auto_child_entity_spawner_component);
    }

    // Spawn thruster entity as child of mob entity

    if let Some(thruster_data) = mob_data.thruster_data {
        let thruster_parent = Parent::new(mob_entity);

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

    mob_entity
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

pub fn spawn_item_drop(
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

    let mut motion2d_component = items_resource.motion2d_component.clone();
    motion2d_component.velocity.x = thread_rng().gen_range(
        items_resource.random_initial_motion.linear.x.0,
        items_resource.random_initial_motion.linear.x.1,
    );
    motion2d_component.velocity.y = thread_rng().gen_range(
        items_resource.random_initial_motion.linear.y.0,
        items_resource.random_initial_motion.linear.y.1,
    );
    motion2d_component.angular_velocity = thread_rng().gen_range(
        items_resource.random_initial_motion.angular.0,
        items_resource.random_initial_motion.angular.1,
    );

    let item_entity = lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(item_data.item_component)
        .with(items_resource.hitbox2d_component.clone())
        .with(motion2d_component)
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
    let effect_data = &effects_resource[effect_type];

    for sprite_render_data in effect_data.sprite_render_data.iter() {
        // for each spriterender data in the array create an entity with a unique sprite render and the same other properties

        let sprite_render = SpriteRender {
            sprite_sheet: spritesheets_resource.spritesheets[&sprite_render_data.spritesheet]
                .clone(),
            sprite_number: sprite_render_data.initial_index,
        };

        let effect_entity = lazy_update
            .create_entity(entities)
            .with(sprite_render)
            .with(spawn_transform.clone())
            .with(Transparent)
            .build();

        // TODO: scale to be a vector (like with Sprite render)
        if let Some(animation_component) = effect_data.animation_component.clone() {
            lazy_update.insert(effect_entity, animation_component);
        }

        if let Some(time_limit_component) = effect_data.time_limit_component.clone() {
            lazy_update.insert(effect_entity, time_limit_component);
        }

        if let Some(mut motion2d_component) = effect_data.motion2d_component.clone() {
            if let Some(random_initial_motion) = effect_data.random_initial_motion.clone() {
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

        if let Some(fade_component) = effect_data.fade_component.clone() {
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
}

pub fn spawn_spawnable(
    spawnable_type: &SpawnableType,
    spawn_transform: Transform,
    consumables_resource: &ReadExpect<ConsumablesResource>,
    mobs_resource: &ReadExpect<MobsResource>,
    items_resource: &ReadExpect<ItemsResource>,
    effects_resource: &ReadExpect<EffectsResource>,
    spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    match spawnable_type {
        SpawnableType::Consumable(consumable_type) => {
            spawn_consumable(
                consumable_type,
                spawn_transform,
                consumables_resource,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }

        SpawnableType::Mob(mob_type) => {
            spawn_mob(
                mob_type,
                spawn_transform,
                mobs_resource,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }

        SpawnableType::Item(item_type) => {
            spawn_item(
                item_type,
                spawn_transform,
                items_resource,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }

        SpawnableType::Effect(effect_type) => {
            spawn_effect(
                effect_type,
                spawn_transform,
                effects_resource,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }
    }
}

pub fn spawn_spawnable_drop(
    spawnable_type: &SpawnableType,
    spawn_transform: Transform,
    consumables_resource: &ReadExpect<ConsumablesResource>,
    mobs_resource: &ReadExpect<MobsResource>,
    items_resource: &ReadExpect<ItemsResource>,
    effects_resource: &ReadExpect<EffectsResource>,
    spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    match spawnable_type {
        SpawnableType::Consumable(consumable_type) => {
            spawn_consumable_drop(
                consumable_type,
                spawn_transform,
                consumables_resource,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }

        SpawnableType::Mob(mob_type) => {
            spawn_mob(
                mob_type,
                spawn_transform,
                mobs_resource,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }

        SpawnableType::Item(item_type) => {
            spawn_item_drop(
                item_type,
                spawn_transform,
                items_resource,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }

        SpawnableType::Effect(effect_type) => {
            spawn_effect(
                effect_type,
                spawn_transform,
                effects_resource,
                spritesheets_resource,
                entities,
                lazy_update,
            );
        }
    }
}

pub fn spawn_random_spawnable(
    drop_probs: &DropProbabilities,
    spawn_transform: Transform,
    consumables_resource: &ReadExpect<ConsumablesResource>,
    mobs_resource: &ReadExpect<MobsResource>,
    items_resource: &ReadExpect<ItemsResource>,
    effects_resource: &ReadExpect<EffectsResource>,
    spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    // choose random spawnable
    spawn_spawnable(
        choose_loot(drop_probs),
        spawn_transform,
        consumables_resource,
        mobs_resource,
        items_resource,
        effects_resource,
        spritesheets_resource,
        entities,
        lazy_update,
    );
}

pub fn spawn_random_drop(
    drop_probs: &DropProbabilities,
    spawn_transform: Transform,
    consumables_resource: &ReadExpect<ConsumablesResource>,
    mobs_resource: &ReadExpect<MobsResource>,
    items_resource: &ReadExpect<ItemsResource>,
    effects_resource: &ReadExpect<EffectsResource>,
    spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    // choose random spawnable
    spawn_spawnable_drop(
        choose_loot(drop_probs),
        spawn_transform,
        consumables_resource,
        mobs_resource,
        items_resource,
        effects_resource,
        spritesheets_resource,
        entities,
        lazy_update,
    );
}

pub fn spawn_drops(
    drop_rolls: &DropRolls,
    spawn_transform: Transform,
    drop_tables_resource: &ReadExpect<DropTablesResource>,
    consumables_resource: &ReadExpect<ConsumablesResource>,
    mobs_resource: &ReadExpect<MobsResource>,
    items_resource: &ReadExpect<ItemsResource>,
    effects_resource: &ReadExpect<EffectsResource>,
    spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    for _ in 0..drop_rolls.roll_count {
        // pick a drop table
        let drop_table = choose_drop_table(&drop_rolls.roll_probs);
        if let DropTableType::NoDrop = drop_table {
        } else {
            spawn_random_drop(
                &drop_tables_resource[drop_table],
                spawn_transform.clone(),
                consumables_resource,
                mobs_resource,
                items_resource,
                effects_resource,
                spritesheets_resource,
                entities,
                lazy_update,
            )
        }
    }
}

pub fn choose_drop_table(roll_probs: &RollProbabilities) -> &DropTableType {
    let prob_space = roll_probs
        .iter()
        .fold(0.0, |sum, roll_prob| sum + roll_prob.1);

    let pos = rand::thread_rng().gen::<f32>() * prob_space;
    let mut sum = 0.0;
    for roll_prob in roll_probs.iter() {
        sum += roll_prob.1;
        if sum > pos {
            return &roll_prob.0;
        }
    }
    unreachable!("Error in probabilities of random spawnable pool.")
}
