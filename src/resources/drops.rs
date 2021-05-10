use crate::entities::SpawnableType;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type RollProbabilities = Vec<(DropTableType, f32)>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DropRolls {
    pub roll_count: u8,
    pub roll_probs: RollProbabilities,
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum DropTableType {
    NoDrop,
    Standard,
    Boss,
}

pub type DropProbabilities = Vec<(SpawnableType, f32)>;

pub type DropTablesResource = HashMap<DropTableType, DropProbabilities>;
