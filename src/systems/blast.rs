use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Entities, Join, System, ReadStorage, WriteStorage, Read};
use amethyst::core::timing::Time;
use crate::space_shooter::{GAME_HEIGHT};
use crate::components::Blast;


pub struct BlastSystem;
impl<'s> System<'s> for BlastSystem {

    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Blast>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, blasts, mut transforms, time): Self::SystemData) {
        for (blast_entity, blast_component, blast_transform) in (&*entities, &blasts, &mut transforms).join() {
            blast_transform.translate_y(blast_component.speed * time.delta_seconds());

            if blast_transform.translation()[1] > GAME_HEIGHT {
                let _result = entities.delete(blast_entity);
            }
        }
    }
}
