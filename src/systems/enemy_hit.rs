use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities,Join, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{Spaceship, Blast},
    systems::hitbox_collide,
};


pub struct EnemyHitSystem;
impl<'s> System<'s> for EnemyHitSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Spaceship>,
        WriteStorage<'s, Blast>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, mut spaceships, mut blasts, transforms): Self::SystemData) {
        for (spaceship, spaceship_transform) in (&mut spaceships, &transforms).join() {

            let spaceship_x = spaceship_transform.translation().x;
            let spaceship_y = spaceship_transform.translation().y;

            for (blast_entity, blast, blast_transform) in (&*entities, &mut blasts, &transforms).join() {

                if !blast.allied {
                    let blast_x = blast_transform.translation().x;
                    let blast_y = blast_transform.translation().y;

                    if hitbox_collide(blast_x, blast_y, spaceship_x, spaceship_y, blast.hitbox_radius, blast.hitbox_radius, spaceship.hitbox_width, spaceship.hitbox_height) && !spaceship.barrel_action_left && !spaceship.barrel_action_right {
                        let _result = entities.delete(blast_entity);
                        spaceship.health -= blast.damage;
                        println!("Player Health: {}", spaceship.health);
                    }
                }
            }
        }
    }
}