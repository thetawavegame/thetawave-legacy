use crate::{
    components::{DespawnAtBottomTag, EnemyComponent, Hitbox2DComponent},
    constants::ARENA_MIN_Y,
    events::EnemyReachedBottomEvent,
};
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System},
    ecs::*,
    shrev::EventChannel,
};

pub struct DespawnAtBottomSystem;

impl<'s> System<'s> for DespawnAtBottomSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, DespawnAtBottomTag>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Hitbox2DComponent>,
        ReadStorage<'s, EnemyComponent>,
        Write<'s, EventChannel<EnemyReachedBottomEvent>>,
    );

    fn run(
        &mut self,
        (
            entities,
            despawn_bottoms,
            transforms,
            hitboxes,
            enemies,
            mut enemy_reached_bottom_event_channel,
        ): Self::SystemData,
    ) {
        for (entity, _despawn_bottom, transform, hitbox) in
            (&*entities, &despawn_bottoms, &transforms, &hitboxes).join()
        {
            if transform.translation().y - hitbox.height / 2.0 < ARENA_MIN_Y {
                if let Some(enemy) = enemies.get(entity) {
                    enemy_reached_bottom_event_channel
                        .single_write(EnemyReachedBottomEvent::new(enemy.defense_damage));
                }
                entities.delete(entity).expect("unable to delete entity");
            }
        }
    }
}
