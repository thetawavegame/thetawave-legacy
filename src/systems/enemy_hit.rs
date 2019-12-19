use amethyst::{
    core::{
        transform::Transform,
        math::Vector3,
    },
    ecs::prelude::{Entities,Join, ReadStorage, System, WriteStorage, ReadExpect, LazyUpdate},
};
use crate::{
    components::{Spaceship, Blast},
    systems::hitbox_collide,
    entities::spawn_blast_explosion,
    constants::EXPLOSION_Z,
    resources::SpriteResource,
};

pub struct EnemyHitSystem;

impl<'s> System<'s> for EnemyHitSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Spaceship>,
        WriteStorage<'s, Blast>,
        ReadStorage<'s, Transform>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut spaceships, mut blasts, transforms, sprite_resource, lazy_update): Self::SystemData) {
        for (spaceship, spaceship_transform) in (&mut spaceships, &transforms).join() {
            for (blast_entity, blast, blast_transform) in (&*entities, &mut blasts, &transforms).join() {

                //first check if the blast is allied with the player
                if !blast.allied {
                    let spaceship_x = spaceship_transform.translation().x;
                    let spaceship_y = spaceship_transform.translation().y;
                    let blast_x = blast_transform.translation().x;
                    let blast_y = blast_transform.translation().y;

                    //if the blast collides with the player and the player is not currently barrel rolling the blast hits
                    if hitbox_collide(blast_x, blast_y, spaceship_x, spaceship_y, blast.hitbox_radius, blast.hitbox_radius, spaceship.hitbox_width, spaceship.hitbox_height) && !spaceship.barrel_action_left && !spaceship.barrel_action_right {
                        let _result = entities.delete(blast_entity);

                        let explosion_position = Vector3::new(
                            blast_transform.translation().x, blast_transform.translation().y, EXPLOSION_Z
                        );

                        spawn_blast_explosion(&entities, sprite_resource.blast_explosions_sprite_sheet.clone(), blast.blast_type.clone(), explosion_position, &lazy_update);

                        spaceship.health -= blast.damage;
                    }
                }
            }
        }
    }
}