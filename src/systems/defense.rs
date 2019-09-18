use amethyst::{
    ecs::prelude::{Join, System, WriteStorage},
};
use crate::{
    components::{Defense, Living},
};

pub struct DefenseSystem;

impl<'s> System<'s> for DefenseSystem {

    type SystemData = WriteStorage<'s, Defense>;

    fn run(&mut self, mut defenses: Self::SystemData) {
        for defense in (&mut defenses).join() {
            defense.constrain_health();
        }

    }
}
