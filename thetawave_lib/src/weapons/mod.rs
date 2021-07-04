//! `thetawave_lib` weapons module

use serde::{Deserialize, Serialize};

mod weapon_components;
mod weapon_systems;

pub use self::{
    weapon_components::{AutoFireComponent, BlasterComponent, ManualFireComponent},
    weapon_systems::{AutoFireSystem, ManualBlasterSystem},
};

/// Used for determining sprite and applied statuses
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BlastType {
    Ally,
    Enemy,
    AllyPoison,
    AllyCritical,
}
