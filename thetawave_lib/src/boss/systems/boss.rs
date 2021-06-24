use crate::boss::components::RepeaterComponent;

use amethyst::ecs::prelude::{Entities, Join, System, WriteStorage};

pub struct BossSystem;

impl<'s> System<'s> for BossSystem {
    type SystemData = (Entities<'s>, WriteStorage<'s, RepeaterComponent>);

    fn run(&mut self, (entities, mut repeaters): Self::SystemData) {
        for (boss_entity, repeater_component) in (&*entities, &mut repeaters).join() {
            if !entities.is_alive(repeater_component.head) {
                entities
                    .delete(boss_entity)
                    .expect("unable to delete entity");

                println!("repeater defeated");
            }
        }
    }
}
