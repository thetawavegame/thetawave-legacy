use crate::{
    constants::{ARENA_MAX_X, ARENA_MAX_Y, ARENA_MIN_X, ARENA_SPAWN_OFFSET, SPAWNER_Y_OFFSET},
    entities::{spawn::spawn_spawnable, SpawnableType},
    resources::{
        ConsumablesResource, EffectsResource, EnemiesResource, InvasionFormationPool,
        InvasionRandomPool, ItemsResource, SpriteSheetsResource,
    },
};
use amethyst::{
    core::{math::Vector2, transform::Transform},
    ecs::prelude::{Entities, LazyUpdate, ReadExpect},
};

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FormationSpawnable {
    pub spawnable_type: SpawnableType,
    pub position: Vector2<f32>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Formation {
    pub formation_spawnables: Vec<FormationSpawnable>,
    pub weight: f32,
    pub period: f32,
}

impl Formation {
    pub fn spawn_formation(
        &self,
        consumables_resource: &ReadExpect<ConsumablesResource>,
        enemies_resource: &ReadExpect<EnemiesResource>,
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
                enemies_resource,
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
pub struct RandomSpawnable {
    pub spawnable_type: Option<SpawnableType>,
    pub weight: f32,
    pub period: f32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SpawnerResource {
    pub random_pools: HashMap<InvasionRandomPool, Vec<RandomSpawnable>>, //TODO: change to HashMap of Vec<SpawnableType> for AsteroidField, Drones, etc.
    pub formation_pools: HashMap<InvasionFormationPool, Vec<Formation>>, //TODO: change to HashMap of Vec<Formation> for Level1Formations, Level2Formations, etc.
    pub timer: f32,
}

impl SpawnerResource {
    fn choose_spawn_position() -> f32 {
        let max_width = ARENA_MAX_X - ARENA_SPAWN_OFFSET;
        let min_width = ARENA_MIN_X + ARENA_SPAWN_OFFSET;
        ARENA_MIN_X + ARENA_SPAWN_OFFSET + rand::thread_rng().gen::<f32>() * (max_width - min_width)
    }

    fn choose_random_spawnable(&self, random_pool_type: &InvasionRandomPool) -> &RandomSpawnable {
        let random_pool = &self.random_pools[random_pool_type];

        let prob_space = random_pool
            .iter()
            .fold(0.0, |sum, rand_spawnable| sum + rand_spawnable.weight);

        let pos = rand::thread_rng().gen::<f32>() * prob_space;
        let mut sum = 0.0;
        for random_spawnable in random_pool.iter() {
            sum += random_spawnable.weight;
            if sum > pos {
                return &random_spawnable;
            }
        }
        unreachable!("Error in probabilities of random spawnable pool.");
    }

    pub fn spawn_random_spawnable_when_ready(
        &mut self,
        random_pool_type: &InvasionRandomPool,
        dt: f32,
        consumables_resource: &ReadExpect<ConsumablesResource>,
        enemies_resource: &ReadExpect<EnemiesResource>,
        items_resource: &ReadExpect<ItemsResource>,
        effects_resource: &ReadExpect<EffectsResource>,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        self.timer -= dt;

        if self.timer <= 0.0 {
            let random_spawnable = self.choose_random_spawnable(random_pool_type);
            let mut spawn_transform = Transform::default();
            spawn_transform.set_translation_xyz(
                Self::choose_spawn_position(),
                ARENA_MAX_Y + SPAWNER_Y_OFFSET,
                0.0,
            );

            if let Some(spawnable_type) = &random_spawnable.spawnable_type {
                spawn_spawnable(
                    spawnable_type,
                    spawn_transform,
                    consumables_resource,
                    enemies_resource,
                    items_resource,
                    effects_resource,
                    spritesheets_resource,
                    entities,
                    lazy_update,
                );
            }

            self.timer = random_spawnable.period;
        }
    }

    fn choose_random_formation(&self, formation_pool_type: &InvasionFormationPool) -> &Formation {
        let formation_pool = &self.formation_pools[formation_pool_type];

        let prob_space = formation_pool
            .iter()
            .fold(0.0, |sum, formation| sum + formation.weight);

        let pos = rand::thread_rng().gen::<f32>() * prob_space;
        let mut sum = 0.0;
        for formation in formation_pool.iter() {
            sum += formation.weight;
            if sum > pos {
                return &formation;
            }
        }
        unreachable!("Error in probabilities of formation pool.");
    }

    pub fn spawn_random_formation_when_ready(
        &mut self,
        formation_pool_type: &InvasionFormationPool,
        dt: f32,
        consumables_resource: &ReadExpect<ConsumablesResource>,
        enemies_resource: &ReadExpect<EnemiesResource>,
        items_resource: &ReadExpect<ItemsResource>,
        effects_resource: &ReadExpect<EffectsResource>,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        self.timer -= dt;

        if self.timer <= 0.0 {
            let formation = self.choose_random_formation(formation_pool_type);
            formation.spawn_formation(
                consumables_resource,
                enemies_resource,
                items_resource,
                effects_resource,
                spritesheets_resource,
                entities,
                lazy_update,
            );

            self.timer = formation.period;
        }
    }
}
