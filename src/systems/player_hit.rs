use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities,Join, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{Enemy, Blast},
    systems::hitbox_collide,
};


pub struct PlayerHitSystem;
impl<'s> System<'s> for PlayerHitSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Blast>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, mut enemies, mut blasts, transforms): Self::SystemData) {
        for (enemy, enemy_transform) in (&mut enemies, &transforms).join() {

            let enemy_x = enemy_transform.translation().x;
            let enemy_y = enemy_transform.translation().y;

            for (blast_entity, blast, blast_transform) in (&*entities, &mut blasts, &transforms).join() {

                let blast_x = blast_transform.translation().x;
                let blast_y = blast_transform.translation().y;

                if hitbox_collide(blast_x, blast_y, enemy_x, enemy_y, blast.hitbox_radius, blast.hitbox_radius, enemy.hitbox_width, enemy.hitbox_height) {
                    let _result = entities.delete(blast_entity);
                    enemy.health -= blast.damage;
                }

            }
        }
    }
}
