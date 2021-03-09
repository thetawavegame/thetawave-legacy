use crate::{
    entities::{spawn_consumable, spawn_effect, spawn_enemy, spawn_item, EntityType},
    resources::{
        ConsumablesResource, EffectsResource, EnemiesResource, ItemsResource, SpriteSheetsResource,
    },
};

use amethyst::{
    core::{
        math::{Vector2, Vector3},
        transform::Transform,
    },
    ecs::prelude::{Component, DenseVecStorage, Entities, LazyUpdate, ReadExpect},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutoChildEntitySpawnerComponent {
    pub child_entity_type: EntityType,
    pub offset: Vector2<f32>,
    period: f32,
    timer: f32,
}

impl Component for AutoChildEntitySpawnerComponent {
    type Storage = DenseVecStorage<Self>;
}

impl AutoChildEntitySpawnerComponent {
    pub fn spawn_when_ready(
        &mut self,
        delta_time: f32,
        spawn_position: Vector3<f32>,
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
            match &self.child_entity_type {
                EntityType::Enemy(enemy_type) => {
                    spawn_enemy(
                        &enemy_type,
                        Vector3::new(
                            spawn_position.x + self.offset.x,
                            spawn_position.y + self.offset.y,
                            spawn_position.z,
                        ),
                        &enemies_resource,
                        &spritesheets_resource,
                        &entities,
                        &lazy_update,
                    );
                }

                EntityType::Consumable(consumable_type) => {
                    spawn_consumable(
                        &consumable_type,
                        Vector3::new(
                            spawn_position.x + self.offset.x,
                            spawn_position.y + self.offset.y,
                            spawn_position.z,
                        ),
                        &consumables_resource,
                        &spritesheets_resource,
                        &entities,
                        &lazy_update,
                    );
                }

                EntityType::Item(item_type) => {
                    spawn_item(
                        &item_type,
                        Vector3::new(
                            spawn_position.x + self.offset.x,
                            spawn_position.y + self.offset.y,
                            spawn_position.z,
                        ),
                        &items_resource,
                        &spritesheets_resource,
                        &entities,
                        &lazy_update,
                    );
                }

                EntityType::Effect(effect_type) => {
                    let mut transform = Transform::default();
                    transform.set_translation_xyz(
                        spawn_position.x + self.offset.x,
                        spawn_position.y + self.offset.y,
                        spawn_position.z,
                    );

                    spawn_effect(
                        &effect_type,
                        transform,
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
