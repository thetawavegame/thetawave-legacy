use crate::{
    components::{BlastComponent, Hitbox2DComponent},
    constants::{ARENA_MAX_X, ARENA_MAX_Y, ARENA_MIN_X, ARENA_MIN_Y},
};
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities, Join, ReadStorage, System, WriteStorage},
};

pub struct BlastSystem;

impl<'s> System<'s> for BlastSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, BlastComponent>,
        ReadStorage<'s, Hitbox2DComponent>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, blasts, hitboxes, mut transforms): Self::SystemData) {
        for (blast_entity, _blast_component, blast_transform, blast_hitbox) in
            (&*entities, &blasts, &mut transforms, &hitboxes).join()
        {
            // delete blast if outside of the arena
            if (blast_transform.translation().y + blast_hitbox.height) > ARENA_MAX_Y
                || (blast_transform.translation().y - blast_hitbox.height) < ARENA_MIN_Y
                || (blast_transform.translation().x + blast_hitbox.width) > ARENA_MAX_X
                || (blast_transform.translation().x - blast_hitbox.width) < ARENA_MIN_X
            {
                entities
                    .delete(blast_entity)
                    .expect("unable to delete entity");
            }
        }
    }
}
