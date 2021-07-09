//! `thetawave_lib` misc module

pub mod components;
pub mod entities;
mod health;
pub mod resources;
pub mod systems;

pub use self::health::{DefenseResource, HealthComponent};
