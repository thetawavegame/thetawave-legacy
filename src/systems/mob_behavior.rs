use crate::{
    components::{HealthComponent, MobComponent},
    events::MobDestroyedEvent,
};
use amethyst::{
    ecs::prelude::{Entities, Join, System, Write, WriteStorage},
    shrev::EventChannel,
};

pub struct MobBehaviorSystem;

impl<'s> System<'s> for MobBehaviorSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, MobComponent>,
        WriteStorage<'s, HealthComponent>,
        Write<'s, EventChannel<MobDestroyedEvent>>,
    );

    fn run(
        &mut self,
        (entities, mut mobs, mut healths, mut mob_destroyed_event_channel): Self::SystemData,
    ) {
        for (mob_entity, _mob_component, mob_health) in (&*entities, &mut mobs, &mut healths).join()
        {
            mob_health.constrain();

            // conditions for despawning
            if mob_health.value <= 0.0 {
                mob_destroyed_event_channel.single_write(MobDestroyedEvent::new(mob_entity));
            }
        }
    }
}
