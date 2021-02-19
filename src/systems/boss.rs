use crate::{components::RepeaterComponent, resources::PhaseManagerResource};
use amethyst::ecs::prelude::{Entities, Join, System, Write, WriteStorage};

pub struct BossSystem;

impl<'s> System<'s> for BossSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, RepeaterComponent>,
        Write<'s, PhaseManagerResource>,
    );

    fn run(&mut self, (entities, mut repeaters, mut phase_managers): Self::SystemData) {
        for (boss_entity, repeater_component) in (&*entities, &mut repeaters).join() {
            if !entities.is_alive(repeater_component.head)
                && !entities.is_alive(repeater_component.body)
            {
                entities
                    .delete(boss_entity)
                    .expect("unable to delete entity");

                phase_managers.phase_idx += 1;

                println!("repeater defeated");
            }
        }
    }
}
