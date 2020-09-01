use crate::components::TimeLimitComponent;
use amethyst::{
    core::timing::Time,
    ecs::prelude::{Entities, Join, Read, System, WriteStorage},
};

pub struct TimeLimitSystem;

impl<'s> System<'s> for TimeLimitSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, TimeLimitComponent>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, mut explosions, time): Self::SystemData) {
        for (timed_entity, time_component) in (&*entities, &mut explosions).join() {
            if time_component.duration > 0.0 {
                time_component.duration -= time.delta_seconds();
            } else {
                let _result = entities.delete(timed_entity);
            }
        }
    }
}
