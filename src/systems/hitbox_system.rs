use crate::{components::Hitbox2DComponent, events::HitboxCollisionEvent};
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities, Join, ReadStorage, System, Write},
    shrev::EventChannel,
};

pub struct HitboxSystem;

impl<'s> System<'s> for HitboxSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Hitbox2DComponent>,
        ReadStorage<'s, Transform>,
        Write<'s, EventChannel<HitboxCollisionEvent>>,
    );

    fn run(&mut self, (entities, hitbox2ds, transforms, mut collision_channel): Self::SystemData) {
        'outer: for (entity_a, transform_a, hitbox_a) in (&entities, &transforms, &hitbox2ds).join()
        {
            for (entity_b, transform_b, hitbox_b) in (&entities, &transforms, &hitbox2ds).join() {
                if entity_a == entity_b {
                    continue;
                }

                if hitbox_a.is_colliding(hitbox_b, transform_a, transform_b) {
                    collision_channel.single_write(HitboxCollisionEvent::new(entity_a, entity_b));
                    break 'outer; // breaks out of outer loop to prevent duplicate events
                }
            }
        }
    }
}
