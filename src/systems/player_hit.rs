use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities,Join, ReadStorage, System, WriteStorage, Read, ReadExpect},
    audio::{output::Output, Source},
    assets::AssetStorage,
};

use std::ops::Deref;

use crate::{
    components::{Enemy, Blast},
    systems::hitbox_collide,
    audio::{play_sfx, Sounds},
};


pub struct PlayerHitSystem;
impl<'s> System<'s> for PlayerHitSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Blast>,
        ReadStorage<'s, Transform>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>
    );

    fn run(&mut self, (entities, mut enemies, mut blasts, transforms, storage, sounds, audio_output): Self::SystemData) {
        for (enemy, enemy_transform) in (&mut enemies, &transforms).join() {
            for (blast_entity, blast, blast_transform) in (&*entities, &mut blasts, &transforms).join() {
                if blast.allied {
                    let enemy_x = enemy_transform.translation().x;
                    let enemy_y = enemy_transform.translation().y;
                    let blast_x = blast_transform.translation().x;
                    let blast_y = blast_transform.translation().y;

                    if hitbox_collide(blast_x, blast_y, enemy_x, enemy_y, blast.hitbox_radius, blast.hitbox_radius, enemy.hitbox_width, enemy.hitbox_height) {
                        let _result = entities.delete(blast_entity);
                        play_sfx(&sounds.spaceship_hit_sfx, &storage, audio_output.as_ref().map(|o| o.deref()));
                        enemy.health -= blast.damage;
                    }
                }
            }
        }
    }
}
