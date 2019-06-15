use amethyst::{
    core::{
        transform::Transform,
        timing::Time,
        nalgebra::Vector3,
    },
    ecs::prelude::{Entities, Join, System, WriteStorage, Read, ReadExpect, LazyUpdate},
};

use rand::{thread_rng, Rng};

use crate::{
    components::{Enemy, Defense, Rigidbody, Fires, EnemyType},
    entities::{spawn_explosion, spawn_consumable},
    resources::SpriteResource,
};
use crate::entities::fire_blast;
use crate::space_shooter::ARENA_MIN_Y;

const ENEMY_BLAST_SPRITE_INDEX: usize = 9;
const EXPLOSION_SPRITE_INDEX: usize = 4;

const EXPLOSION_Z: f32 = 0.0;


pub struct EnemySystem;
impl<'s> System<'s> for EnemySystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Enemy>,
        WriteStorage<'s, Defense>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut enemys, mut defenses, mut transforms, time, sprite_resource, lazy_update): Self::SystemData) {
        for (enemy_entity, enemy_component, enemy_transform) in (&*entities, &mut enemys, &mut transforms).join() {

            //limit the maximum knockback and speed
            enemy_component.limit_knockback();
            enemy_component.limit_speed();

            //constrain in arena
            enemy_component.constrain_to_arena(enemy_transform); 

            //accelerate in -y direction
            enemy_component.accelerate(0.0, -1.0);

            //transform the spaceship in x and y by the currrent velocity in x and y
            enemy_component.update_position(enemy_transform, time.delta_seconds());

            //conditions for despawning
            if enemy_transform.translation()[1] + enemy_component.height/2.0 < ARENA_MIN_Y {
                
                //defense is damage is enemy gets past
                for defense in (&mut defenses).join() {
                    defense.defense -= enemy_component.defense_damage;
                }
                let _result = entities.delete(enemy_entity);
            }else if enemy_component.health < 0.0 {
                
                //enemy us deleted, explosion is spawned and item dropped
                let death_position = Vector3::new(
                    enemy_transform.translation()[0], enemy_transform.translation()[1], EXPLOSION_Z,
                );
                
                let _result = entities.delete(enemy_entity);

                spawn_explosion(&entities, &sprite_resource, EXPLOSION_SPRITE_INDEX,death_position, &lazy_update);

                if thread_rng().gen::<f32>() < enemy_component.drop_chance {
                    spawn_consumable(&entities, &sprite_resource, &mut enemy_component.consumable_pool, death_position, &lazy_update);
                }
            }

            //behavior for enemies based on its enemy_type attribute
            match enemy_component.enemy_type {
                EnemyType::Pawn => {
                    if let Some(fire_position) = enemy_component.fire_cooldown(enemy_transform, -1.0 * enemy_component.height / 2.0, true, time.delta_seconds()) {
                        fire_blast(&entities, &sprite_resource, ENEMY_BLAST_SPRITE_INDEX, fire_position, enemy_component.blast_damage, 0.0, 0.0, enemy_component.blast_speed, false, &lazy_update);
                    }
                }

                EnemyType::Drone => {}
            }
        }
    }
}