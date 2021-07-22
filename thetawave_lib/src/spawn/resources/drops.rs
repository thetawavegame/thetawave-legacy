use crate::{
    spawnable::{spawn_spawnable, SpawnableResources, SpawnableType},
    tools::weighted_rng,
    visual::SpriteSheetsResource,
};
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities, LazyUpdate},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// `DropTableTypes` mapped to `DropProbabilities`
pub type DropTablesResource = HashMap<DropTableType, DropProbabilities>;
/// Vector of spawnable types paired with random weights
pub type DropProbabilities = Vec<(SpawnableType, f32)>;
/// Vector of optional `DropTableTypes` paired with random weights
pub type RollProbabilities = Vec<(Option<DropTableType>, f32)>;

/// Used for rolling for drops
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DropRolls {
    /// Number of rolls
    pub roll_count: u8,
    /// Optional drop table type paired with weights
    pub roll_probs: RollProbabilities,
}

impl DropRolls {
    /// Choose a drop table from roll probs
    fn choose_drop_table(&self) -> &Option<DropTableType> {
        let probs = self
            .roll_probs
            .iter()
            .map(|roll_prob| roll_prob.1)
            .collect();
        &self.roll_probs[weighted_rng(probs)].0
    }

    /// Choose a drop from drop table (`DropProbabilities`)
    fn choose_drop(drop_probs: &DropProbabilities) -> &SpawnableType {
        let probs = drop_probs.iter().map(|drop_prob| drop_prob.1).collect();
        &drop_probs[weighted_rng(probs)].0
    }

    /// Roll and spawn drops
    pub fn spawn(
        &self,
        spawn_transform: &Transform,
        drop_tables_resource: &DropTablesResource,
        spawnable_resources: &SpawnableResources,
        spritesheets_resource: &SpriteSheetsResource,
        entities: &Entities,
        lazy_update: &LazyUpdate,
    ) {
        for _ in 0..self.roll_count {
            // pick a drop table
            if let Some(drop_table) = self.choose_drop_table() {
                // spawn a drop from the table
                spawn_spawnable(
                    Self::choose_drop(&drop_tables_resource[drop_table]),
                    true,
                    spawn_transform,
                    spawnable_resources,
                    spritesheets_resource,
                    entities,
                    lazy_update,
                )
            }
        }
    }
}

/// Types of drop tables
#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum DropTableType {
    Standard,
    Boss,
    MoneyAsteroid,
}
