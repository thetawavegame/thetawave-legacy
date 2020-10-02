use crate::{
    components::{Hitbox2DComponent, Item, Motion2DComponent},
    constants::ARENA_MIN_Y,
};
use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Entities, Join, WriteStorage},
    ecs::*,
    ecs::{Read, System},
};

#[derive(Default)]
pub struct ItemSystem;

impl<'s> System<'s> for ItemSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Item>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Hitbox2DComponent>,
        ReadStorage<'s, Motion2DComponent>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (entities, mut items, mut transforms, hitboxes, motion2ds, time): Self::SystemData,
    ) {
        // item movement
        for (_item, item_entity, item_transform, item_hitbox, item_motion2d) in (
            &mut items,
            &entities,
            &mut transforms,
            &hitboxes,
            &motion2ds,
        )
            .join()
        {
            item_transform
                .prepend_translation_y(-1.0 * item_motion2d.velocity.y * time.delta_seconds());

            if item_transform.translation().y + item_hitbox.height / 2.0 < ARENA_MIN_Y {
                entities
                    .delete(item_entity)
                    .expect("unable to delete entity");
            }
        }
    }
}
