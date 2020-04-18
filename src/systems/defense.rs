use crate::components::{Defense, Living};
use amethyst::ecs::prelude::{Join, System, WriteStorage};

pub struct DefenseSystem;

impl<'s> System<'s> for DefenseSystem {
    type SystemData = WriteStorage<'s, Defense>;

    fn run(&mut self, mut defenses: Self::SystemData) {
        for defense in (&mut defenses).join() {
            defense.constrain_health();
        }
    }
}
