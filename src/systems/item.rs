use crate::{
    components::{Hitbox2DComponent, ItemComponent},
    constants::ARENA_MIN_Y,
};
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities, Join, WriteStorage},
    ecs::System,
    ecs::*,
};

#[derive(Default)]
pub struct ItemSystem;

impl<'s> System<'s> for ItemSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, ItemComponent>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Hitbox2DComponent>,
    );

    fn run(&mut self, (entities, mut items, mut transforms, hitboxes): Self::SystemData) {
        // item movement
        for (_item, item_entity, item_transform, item_hitbox) in
            (&mut items, &entities, &mut transforms, &hitboxes).join()
        {
            if item_transform.translation().y + item_hitbox.height / 2.0 < ARENA_MIN_Y {
                entities
                    .delete(item_entity)
                    .expect("unable to delete entity");
            }
        }
    }
}
