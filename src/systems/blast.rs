use crate::{
    components::{BlastComponent, Hitbox2DComponent, Motion2DComponent},
    constants::{ARENA_MAX_X, ARENA_MAX_Y, ARENA_MIN_X, ARENA_MIN_Y},
};
use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
};

pub struct BlastSystem;

impl<'s> System<'s> for BlastSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, BlastComponent>,
        ReadStorage<'s, Hitbox2DComponent>,
        ReadStorage<'s, Motion2DComponent>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (entities, blasts, hitboxes, motion2ds, mut transforms, time): Self::SystemData,
    ) {
        for (blast_entity, _blast_component, blast_transform, blast_hitbox, blast_motion2d) in
            (&*entities, &blasts, &mut transforms, &hitboxes, &motion2ds).join()
        {
            // delete blast if outside of the arena
            // TODO add hitbox to side panel
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
