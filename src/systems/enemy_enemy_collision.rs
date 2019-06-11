use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities,Join, ReadStorage, System, WriteStorage},
};

use crate::{
    components::{Enemy},
    systems::hitbox_collide,
};

pub struct EnemyEnemyCollisionSystem;
impl<'s> System<'s> for EnemyEnemyCollisionSystem {

    type SystemData = (
        WriteStorage<'s, Enemy>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut enemies, transforms): Self::SystemData) {
        /*
        for (enemy, enemy_transform) in (&mut enemies, &transforms).join() {
            if let Some(enemy) = prev_enemy {
                println!("prev enemy: {}", enemy.damage);
            }
            prev_enemy = enemy;
        }
        */
    }
}