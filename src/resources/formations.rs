use crate::{
    entities::{spawn::spawn_spawnable, SpawnableType},
    resources::{
        ConsumablesResource, EffectsResource, ItemsResource, MobsResource, SpriteSheetsResource,
    },
};
use amethyst::{
    core::{math::Vector2, transform::Transform},
    ecs::prelude::{Entities, LazyUpdate, ReadExpect},
};

use rand::seq::SliceRandom;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FormationSpawnable {
    pub spawnable_type: SpawnableType,
    pub position: Vector2<f32>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Formation {
    pub formation_spawnables: Vec<FormationSpawnable>,
}

impl Formation {
    pub fn spawn_formation(
        &self,
        consumables_resource: &ReadExpect<ConsumablesResource>,
        mobs_resource: &ReadExpect<MobsResource>,
        items_resource: &ReadExpect<ItemsResource>,
        effects_resource: &ReadExpect<EffectsResource>,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        for formation_spawnable in self.formation_spawnables.iter() {
            let mut spawn_transform = Transform::default();
            spawn_transform.set_translation_xyz(
                formation_spawnable.position.x,
                formation_spawnable.position.y,
                0.0,
            );

            spawn_spawnable(
                &formation_spawnable.spawnable_type,
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
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FormationsResource {
    pub formations: Vec<Formation>,
    pub period: f32,
    pub timer: f32,
}

impl FormationsResource {
    pub fn spawn_random_formation_when_ready(
        &mut self,
        dt: f32,
        consumables_resource: &ReadExpect<ConsumablesResource>,
        mobs_resource: &ReadExpect<MobsResource>,
        items_resource: &ReadExpect<ItemsResource>,
        effects_resource: &ReadExpect<EffectsResource>,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        self.timer -= dt;

        if self.timer <= 0.0 {
            self.timer = self.period;

            self.formations
                .choose(&mut rand::thread_rng())
                .unwrap()
                .spawn_formation(
                    consumables_resource,
                    mobs_resource,
                    items_resource,
                    effects_resource,
                    spritesheets_resource,
                    entities,
                    lazy_update,
                );
        }
    }
}
