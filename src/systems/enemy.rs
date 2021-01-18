use crate::constants::ARENA_HEIGHT;
use crate::{
    components::{
        EnemyComponent, EnemyType, HealthComponent, Hitbox2DComponent, Motion2DComponent,
    },
    constants::ARENA_MIN_Y,
    events::{EnemyDestroyedEvent, EnemyReachedBottomEvent},
};
use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage},
    shrev::EventChannel,
};

pub struct EnemySystem;

impl<'s> System<'s> for EnemySystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, EnemyComponent>,
        WriteStorage<'s, HealthComponent>,
        Write<'s, EventChannel<EnemyDestroyedEvent>>,
    );

    fn run(
        &mut self,
        (entities, mut enemies, mut healths, mut enemy_destroyed_event_channel): Self::SystemData,
    ) {
        for (enemy_entity, enemy_component, enemy_health) in
            (&*entities, &mut enemies, &mut healths).join()
        {
            // TODO: put in a new PoisonSystem
            enemy_health.value -= enemy_component.poison;
            enemy_health.constrain();

            // conditions for despawning
            if enemy_health.value <= 0.0 {
                enemy_destroyed_event_channel.single_write(EnemyDestroyedEvent::new(enemy_entity));
            }
        }
    }
}
