use amethyst::{
    ecs::prelude::{Entities, Join, System, WriteStorage},
};

use crate::{
    components::{Defense, Living},
};


pub struct DefenseSystem;
impl<'s> System<'s> for DefenseSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Defense>,
    );

    fn run(&mut self, (entities, mut defenses): Self::SystemData) {
        for defense in (&mut defenses).join() {
            defense.constrain_health()
        }

    }
}
