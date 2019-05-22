use amethyst::{
    core::{
        transform::Transform,
        timing::Time,
    },
    ecs::prelude::{Entities, Join, System, ReadStorage, WriteStorage, Read},
};

use crate::components::Enemy;


pub struct EnemySystem;
impl<'s> System<'s> for EnemySystem {

    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Enemy>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, enemys, mut transforms, time): Self::SystemData) {

        for (enemy_entity, enemy_component, enemy_transform) in (&*entities, &enemys, &mut transforms).join() {
            enemy_transform.translate_y(-1.0 * enemy_component.speed * time.delta_seconds());

            if enemy_transform.translation()[1] < 0.0 {
                let _result = entities.delete(enemy_entity);
            }
        }

    }
}