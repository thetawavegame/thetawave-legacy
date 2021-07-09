//! `thetawave_lib` boss module

mod components;
mod entities;
mod systems;

pub use self::{components::RepeaterComponent, entities::spawn_repeater, systems::BossSystem};
