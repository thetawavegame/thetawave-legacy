use crate::components::{GameMasterComponent, RepeaterComponent};
use amethyst::ecs::prelude::{Entities, Join, System, WriteStorage};

pub struct BossSystem;

impl<'s> System<'s> for BossSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, RepeaterComponent>,
        WriteStorage<'s, GameMasterComponent>,
    );

    fn run(&mut self, (entities, mut repeaters, mut gamemasters): Self::SystemData) {
        for (boss_entity, repeater_component) in (&*entities, &mut repeaters).join() {
            if !entities.is_alive(repeater_component.head)
                && !entities.is_alive(repeater_component.body)
            {
                entities
                    .delete(boss_entity)
                    .expect("unable to delete entity");

                for gamemaster in (&mut gamemasters).join() {
                    gamemaster.phase_idx += 1;
                }

                println!("repeater defeated");
            }
        }
    }
}
