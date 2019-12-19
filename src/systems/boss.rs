use amethyst::{
    core::{
        timing::Time,
    },
    ecs::prelude::{Entities, Join, System, WriteStorage, Read},
};
use crate::{
    components::Repeater,
};

pub struct BossSystem;

impl<'s> System<'s> for BossSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Repeater>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, mut repeaters, time): Self::SystemData) {
        for (boss_entity, repeater_component) in (&*entities, &mut repeaters).join() {


        }
    }
}