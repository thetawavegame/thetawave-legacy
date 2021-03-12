use crate::{
    entities::{
        spawn_consumable, spawn_effect, spawn_enemy, spawn_item, ConsumableType, EffectType,
        EnemyType, ItemType, SpawnableType,
    },
    resources::{
        ConsumablesResource, EffectsResource, EnemiesResource, ItemsResource, SpriteSheetsResource,
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
        enemies_resource: &ReadExpect<EnemiesResource>,
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
                SpawnableType::Enemy(enemy_type) => {
                    spawn_enemy(
                        &enemy_type,
                        adjusted_transform,
                        &enemies_resource,
                        &spritesheets_resource,
                        &entities,
                        &lazy_update,
                    );
                }

                SpawnableType::Consumable(consumable_type) => {
                    spawn_consumable(
                        &consumable_type,
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
pub struct AutoChildEnemySpawnerComponent {
    pub child_enemy_type: EnemyType,
    pub offset: Vector2<f32>,
    period: f32,
    timer: f32,
}

impl Component for AutoChildEnemySpawnerComponent {
    type Storage = DenseVecStorage<Self>;
}

impl AutoChildEnemySpawnerComponent {
    pub fn spawn_when_ready(
        &mut self,
        delta_time: f32,
        spawn_transform: Transform,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        enemies_resource: &ReadExpect<EnemiesResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        self.timer -= delta_time;

        if self.timer < 0.0 {
            self.timer = self.period;

            let mut adjusted_transform = spawn_transform;
            adjusted_transform.prepend_translation_x(self.offset.x);
            adjusted_transform.prepend_translation_y(self.offset.y);

            spawn_enemy(
                &self.child_enemy_type,
                adjusted_transform,
                &enemies_resource,
                &spritesheets_resource,
                &entities,
                &lazy_update,
            );
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutoChildConsumableSpawnerComponent {
    pub child_consumable_type: ConsumableType,
    pub offset: Vector2<f32>,
    period: f32,
    timer: f32,
}

impl Component for AutoChildConsumableSpawnerComponent {
    type Storage = DenseVecStorage<Self>;
}

impl AutoChildConsumableSpawnerComponent {
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
pub struct AutoChildItemSpawnerComponent {
    pub child_item_type: ItemType,
    pub offset: Vector2<f32>,
    period: f32,
    timer: f32,
}

impl Component for AutoChildItemSpawnerComponent {
    type Storage = DenseVecStorage<Self>;
}

impl AutoChildItemSpawnerComponent {
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
pub struct AutoChildEffectSpawnerComponent {
    pub child_effect_type: EffectType,
    pub offset: Vector2<f32>,
    period: f32,
    timer: f32,
}

impl Component for AutoChildEffectSpawnerComponent {
    type Storage = DenseVecStorage<Self>;
}

impl AutoChildEffectSpawnerComponent {
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
