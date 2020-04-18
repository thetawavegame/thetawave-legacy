use crate::components::Explosion;
use amethyst::{
    core::timing::Time,
    ecs::prelude::{Entities, Join, Read, System, WriteStorage},
};

pub struct ExplosionSystem;

impl<'s> System<'s> for ExplosionSystem {
    type SystemData = (Entities<'s>, WriteStorage<'s, Explosion>, Read<'s, Time>);

    fn run(&mut self, (entities, mut explosions, time): Self::SystemData) {
        for (explosion_entity, explosion_component) in (&*entities, &mut explosions).join() {
            if explosion_component.duration > 0.0 {
                explosion_component.duration -= time.delta_seconds();
            } else {
                let _result = entities.delete(explosion_entity);
            }
        }
    }
}
