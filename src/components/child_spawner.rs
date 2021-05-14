use crate::{
    entities::{
        spawn_consumable, spawn_effect, spawn_item, spawn_mob, ConsumableType, EffectType,
        ItemType, MobType, SpawnableType,
    },
    resources::{
        ConsumablesResource, EffectsResource, ItemsResource, MobsResource, SpriteSheetsResource,
    },
};

use amethyst::{
    core::{math::Vector2, transform::Transform},
    ecs::prelude::{Component, DenseVecStorage, Entities, LazyUpdate, ReadExpect},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutoSpawnerComponent {
    pub child_entity_type: SpawnableType,
    pub offset: Vector2<f32>,
    period: f32,
    timer: f32,
}

impl Component for AutoSpawnerComponent {
    type Storage = DenseVecStorage<Self>;
}

impl AutoSpawnerComponent {
    pub fn spawn_when_ready(
        &mut self,
        delta_time: f32,
        spawn_transform: Transform,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        mobs_resource: &ReadExpect<MobsResource>,
        consumables_resource: &ReadExpect<ConsumablesResource>,
        items_resource: &ReadExpect<ItemsResource>,
        effects_resource: &ReadExpect<EffectsResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        self.timer -= delta_time;

        if self.timer < 0.0 {
            self.timer = self.period;

            let mut adjusted_transform = spawn_transform;
            adjusted_transform.prepend_translation_x(self.offset.x);
            adjusted_transform.prepend_translation_y(self.offset.y);

            match &self.child_entity_type {
                SpawnableType::Mob(mob_type) => {
                    spawn_mob(
                        &mob_type,
                        adjusted_transform,
                        &mobs_resource,
                        &spritesheets_resource,
                        &entities,
                        &lazy_update,
                    );
                }

                SpawnableType::Consumable(consumable_type) => {
                    spawn_consumable(
                        &consumable_type,
                        false,
                        adjusted_transform,
                        &consumables_resource,
                        &spritesheets_resource,
                        &entities,
                        &lazy_update,
                    );
                }

                SpawnableType::Item(item_type) => {
                    spawn_item(
                        &item_type,
                        false,
                        adjusted_transform,
                        &items_resource,
                        &spritesheets_resource,
                        &entities,
                        &lazy_update,
                    );
                }

                SpawnableType::Effect(effect_type) => {
                    spawn_effect(
                        &effect_type,
                        adjusted_transform,
                        &effects_resource,
                        &spritesheets_resource,
                        &entities,
                        &lazy_update,
                    );
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutoMobSpawnerComponent {
    pub child_mob_type: MobType,
    pub offset: Vector2<f32>,
    period: f32,
    timer: f32,
}

impl Component for AutoMobSpawnerComponent {
    type Storage = DenseVecStorage<Self>;
}

impl AutoMobSpawnerComponent {
    #[allow(dead_code)]
    pub fn spawn_when_ready(
        &mut self,
        delta_time: f32,
        spawn_transform: Transform,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        mobs_resource: &ReadExpect<MobsResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        self.timer -= delta_time;

        if self.timer < 0.0 {
            self.timer = self.period;

            let mut adjusted_transform = spawn_transform;
            adjusted_transform.prepend_translation_x(self.offset.x);
            adjusted_transform.prepend_translation_y(self.offset.y);

            spawn_mob(
                &self.child_mob_type,
                adjusted_transform,
                &mobs_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutoConsumableSpawnerComponent {
    pub child_consumable_type: ConsumableType,
    pub offset: Vector2<f32>,
    period: f32,
    timer: f32,
}

impl Component for AutoConsumableSpawnerComponent {
    type Storage = DenseVecStorage<Self>;
}

impl AutoConsumableSpawnerComponent {
    #[allow(dead_code)]
    pub fn spawn_when_ready(
        &mut self,
        delta_time: f32,
        spawn_transform: Transform,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        consumables_resource: &ReadExpect<ConsumablesResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        self.timer -= delta_time;

        if self.timer < 0.0 {
            self.timer = self.period;

            let mut adjusted_transform = spawn_transform;
            adjusted_transform.prepend_translation_x(self.offset.x);
            adjusted_transform.prepend_translation_y(self.offset.y);

            spawn_consumable(
                &self.child_consumable_type,
                false,
                adjusted_transform,
                &consumables_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutoItemSpawnerComponent {
    pub child_item_type: ItemType,
    pub offset: Vector2<f32>,
    period: f32,
    timer: f32,
}

impl Component for AutoItemSpawnerComponent {
    type Storage = DenseVecStorage<Self>;
}

impl AutoItemSpawnerComponent {
    #[allow(dead_code)]
    pub fn spawn_when_ready(
        &mut self,
        delta_time: f32,
        spawn_transform: Transform,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        items_resource: &ReadExpect<ItemsResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        self.timer -= delta_time;

        if self.timer < 0.0 {
            self.timer = self.period;

            let mut adjusted_transform = spawn_transform;
            adjusted_transform.prepend_translation_x(self.offset.x);
            adjusted_transform.prepend_translation_y(self.offset.y);

            spawn_item(
                &self.child_item_type,
                false,
                adjusted_transform,
                &items_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutoEffectSpawnerComponent {
    pub child_effect_type: EffectType,
    pub offset: Vector2<f32>,
    period: f32,
    timer: f32,
}

impl Component for AutoEffectSpawnerComponent {
    type Storage = DenseVecStorage<Self>;
}

impl AutoEffectSpawnerComponent {
    #[allow(dead_code)]
    pub fn spawn_when_ready(
        &mut self,
        delta_time: f32,
        spawn_transform: Transform,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        effects_resource: &ReadExpect<EffectsResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        self.timer -= delta_time;

        if self.timer < 0.0 {
            self.timer = self.period;

            let mut adjusted_transform = spawn_transform;
            adjusted_transform.prepend_translation_x(self.offset.x);
            adjusted_transform.prepend_translation_y(self.offset.y);

            spawn_effect(
                &self.child_effect_type,
                adjusted_transform,
                &effects_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );
        }
    }
}
