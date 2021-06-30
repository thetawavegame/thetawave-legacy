//! `thetawave_lib` weapons module

use serde::{Deserialize, Serialize};

mod components;
mod systems;

pub use self::{
    components::{AutoFireComponent, BlasterComponent, ManualFireComponent},
    systems::{AutoFireSystem, ManualBlasterSystem},
};

/// Used for determining sprite and applied statuses
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BlastType {
    Ally,
    Enemy,
    AllyPoison,
    AllyCritical,
}
