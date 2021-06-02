use crate::{
    components::{DespawnAtBorderComponent, MobComponent},
    constants::{ARENA_MAX_X, ARENA_MAX_Y, ARENA_MIN_X, ARENA_MIN_Y},
    events::MobReachedBottomEvent,
};
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System},
    ecs::*,
    shrev::EventChannel,
};

pub struct DespawnAtBorderSystem;

impl<'s> System<'s> for DespawnAtBorderSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, DespawnAtBorderComponent>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, MobComponent>,
        Write<'s, EventChannel<MobReachedBottomEvent>>,
    );

    fn run(
        &mut self,
        (entities, despawn_borders, transforms, mobs, mut mob_reached_bottom_event_channel): Self::SystemData,
    ) {
        for (entity, despawn_border, transform) in
            (&*entities, &despawn_borders, &transforms).join()
        {
            if let Some(top_border_offset) = despawn_border.top_offset {
                if transform.translation().y > ARENA_MAX_Y + top_border_offset {
                    entities.delete(entity).expect("unable to delete entity");
                }
            }

            if let Some(bottom_border_offset) = despawn_border.bottom_offset {
                if transform.translation().y < ARENA_MIN_Y + bottom_border_offset {
                    if let Some(mob) = mobs.get(entity) {
                        mob_reached_bottom_event_channel
                            .single_write(MobReachedBottomEvent::new(mob.defense_damage));
                    }
                    entities.delete(entity).expect("unable to delete entity");
                }
            }

            if let Some(left_border_offset) = despawn_border.left_offset {
                if transform.translation().x < ARENA_MIN_X + left_border_offset {
                    entities.delete(entity).expect("unable to delete entity");
                }
            }

            if let Some(right_border_offset) = despawn_border.right_offset {
                if transform.translation().x > ARENA_MAX_X + right_border_offset {
                    entities.delete(entity).expect("unable to delete entity");
                }
            }
        }
    }
}
