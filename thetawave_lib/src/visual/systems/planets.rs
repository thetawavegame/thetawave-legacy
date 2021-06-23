use crate::components::PlanetComponent;
use amethyst::{
    core::Transform,
    ecs::prelude::{Join, System, WriteStorage},
};

/// Handles planets
pub struct PlanetsSystem;

impl<'s> System<'s> for PlanetsSystem {
    /// Data used by the system
    type SystemData = (
        WriteStorage<'s, PlanetComponent>,
        WriteStorage<'s, Transform>,
    );

    /// System game logic
    fn run(&mut self, (mut planets, mut transforms): Self::SystemData) {
        for (planet, transform) in (&mut planets, &mut transforms).join() {
            planet.rotate(transform);
        }
    }
}
