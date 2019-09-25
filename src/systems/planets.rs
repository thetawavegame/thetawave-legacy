use amethyst::{
    ecs::prelude::{Join, System, WriteStorage},
    core::Transform,
};
use crate::{
    components::{Planet},
};

pub struct PlanetsSystem;

impl<'s> System<'s> for PlanetsSystem {

    type SystemData = (
        WriteStorage<'s, Planet>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (mut planets, mut transforms): Self::SystemData) {
        for (planet, transform) in (&mut planets, &mut transforms).join() {
            planet.rotate(transform);
        }

    }
}
