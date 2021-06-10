//! `thetawave_lib` weapons module

use serde::{Deserialize, Serialize};

pub mod components;
pub mod systems;

/// Used for determining sprite and applied statuses
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BlastType {
    Ally,
    Enemy,
    AllyPoison,
    AllyCritical,
}
