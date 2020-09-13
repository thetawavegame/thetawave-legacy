use crate::components::Planet;
use amethyst::{
    core::Transform,
    ecs::prelude::{Join, System, WriteStorage},
};

pub struct PlanetsSystem;

impl<'s> System<'s> for PlanetsSystem {
    type SystemData = (WriteStorage<'s, Planet>, WriteStorage<'s, Transform>);

    fn run(&mut self, (mut planets, mut transforms): Self::SystemData) {
        for (planet, transform) in (&mut planets, &mut transforms).join() {
            planet.rotate(transform);
        }
    }
}
