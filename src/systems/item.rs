use crate::{
    components::{Hitbox2DComponent, Item},
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
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, mut items, mut transforms, hitboxes, time): Self::SystemData) {
        // item movement
        for (item, item_entity, item_transform, item_hitbox) in
            (&mut items, &entities, &mut transforms, &hitboxes).join()
        {
            item_transform.prepend_translation_y(-1.0 * item.speed * time.delta_seconds());

            if item_transform.translation().y + item_hitbox.height / 2.0 < ARENA_MIN_Y {
                entities
                    .delete(item_entity)
                    .expect("unable to delete entity");
            }
        }
    }
}
