use crate::components::{GameMaster, Repeater};
use amethyst::{
    core::timing::Time,
    ecs::prelude::{Entities, Entity, Join, Read, System, WriteStorage},
};

pub struct BossSystem;

impl<'s> System<'s> for BossSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Repeater>,
        WriteStorage<'s, GameMaster>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, mut repeaters, mut gamemasters, time): Self::SystemData) {
        for (boss_entity, repeater_component) in (&*entities, &mut repeaters).join() {
            if !entities.is_alive(repeater_component.head)
                && !entities.is_alive(repeater_component.body)
            {
                let _result = entities.delete(boss_entity);

                for gamemaster in (&mut gamemasters).join() {
                    gamemaster.phase_idx += 1;
                }

                println!("repeater defeated");
            }
        }
    }
}
