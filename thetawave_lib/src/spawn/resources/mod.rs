//! Resources for spawning and despawning entities

mod drops;
mod spawner;

pub use self::{
    drops::{DropProbabilities, DropRolls, DropTableType, DropTablesResource, RollProbabilities},
    spawner::{Formation, FormationSpawnable, RandomSpawnable, SpawnerResource},
};
