use crate::{
    components::{Blast, Hitbox2DComponent},
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
        ReadStorage<'s, Blast>,
        ReadStorage<'s, Hitbox2DComponent>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, blasts, hitboxes, mut transforms, time): Self::SystemData) {
        for (blast_entity, blast_component, blast_transform, blast_hitbox) in
            (&*entities, &blasts, &mut transforms, &hitboxes).join()
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

            // update position based on blast velocity
            blast_transform.prepend_translation_x(
                blast_component.x_velocity * blast_component.velocity_factor * time.delta_seconds(),
            );
            blast_transform.prepend_translation_y(
                (blast_component.y_velocity * blast_component.velocity_factor
                    + blast_component.speed)
                    * time.delta_seconds(),
            );
        }
    }
}
