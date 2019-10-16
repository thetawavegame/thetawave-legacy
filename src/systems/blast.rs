use amethyst::{
    core::{
        transform::Transform,
        timing::Time,
    },
    ecs::prelude::{Entities, Join, System, ReadStorage, WriteStorage, Read},
};
use crate::{
    components::Blast,
    constants::{ARENA_MIN_X, ARENA_MAX_X, ARENA_MIN_Y, ARENA_MAX_Y},
};

pub struct BlastSystem;

impl<'s> System<'s> for BlastSystem {

    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Blast>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, blasts, mut transforms, time): Self::SystemData) {
        for (blast_entity, blast_component, blast_transform) in (
            &*entities,
            &blasts,
            &mut transforms
        ).join() {

            // delete blast if outside of the arena
            if  (blast_transform.translation()[1] + blast_component.hitbox_radius) > ARENA_MAX_Y ||
                (blast_transform.translation()[1] - blast_component.hitbox_radius) < ARENA_MIN_Y ||
                (blast_transform.translation()[0] + blast_component.hitbox_radius) > ARENA_MAX_X ||
                (blast_transform.translation()[0] - blast_component.hitbox_radius) < ARENA_MIN_X
            {
                let _result = entities.delete(blast_entity);
            }

            // update position based on blast velocity
            blast_transform.prepend_translation_x(blast_component.x_velocity * blast_component.velocity_factor * time.delta_seconds());
            blast_transform.prepend_translation_y((blast_component.y_velocity * blast_component.velocity_factor + blast_component.speed) * time.delta_seconds());
        }
    }
}
