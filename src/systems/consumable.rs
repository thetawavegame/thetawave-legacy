use crate::{
    components::{Consumable, Hitbox2DComponent},
    constants::ARENA_MIN_Y,
};
use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
};

#[derive(Default)]
pub struct ConsumableSystem;

impl<'s> System<'s> for ConsumableSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Consumable>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Hitbox2DComponent>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (entities, mut consumables, mut transforms, hitboxes, time): Self::SystemData,
    ) {
        for (consumable, consumable_entity, consumable_transform, consumable_hitbox) in
            (&mut consumables, &entities, &mut transforms, &hitboxes).join()
        {
            consumable_transform
                .prepend_translation_y(-1.0 * consumable.speed * time.delta_seconds());

            if consumable_transform.translation().y + consumable_hitbox.height / 2.0 < ARENA_MIN_Y {
                entities
                    .delete(consumable_entity)
                    .expect("unable to delete entity");
            }
        }
    }
}
