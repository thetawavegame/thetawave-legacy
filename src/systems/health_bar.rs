use amethyst::{
    core::{
        nalgebra::Vector3,
    },
    ecs::prelude::{Entities, Join, System, WriteStorage, ReadStorage, ReadExpect, LazyUpdate},
};

use crate::{
    components::{Spaceship, HealthBar},
    entities::spawn_health_unit,
    resources::SpriteResource,
};



pub struct HealthBarSystem;
impl<'s> System<'s> for HealthBarSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, HealthBar>,
        ReadStorage<'s, Spaceship>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut health_bars, spaceships, sprite_resource, lazy_update): Self::SystemData) {
        for health_bar in (&mut health_bars).join() {

            for spaceship in (&spaceships).join() {
                let health_divisor = spaceship.max_health/63.0;
                let mut health_unit_num = ((spaceship.health) / health_divisor).ceil() as usize;
                if spaceship.health <= 0.0 {
                    health_unit_num = 0;
                }

                //push units on
                if health_unit_num > health_bar.health_stack.len() {
                    let health_position = Vector3::new(
                        health_bar.x_pos, health_bar.y_pos, 0.9,
                    );
                    health_bar.y_pos += 1.0;
                    health_bar.health_stack.push(spawn_health_unit(&entities, &sprite_resource, 10, health_position, &lazy_update));
                }

                if health_unit_num < health_bar.health_stack.len() {
                    if let Some(unit) = health_bar.health_stack.pop() {
                        let _result = entities.delete(unit);
                        health_bar.y_pos -= 1.0;
                    }

                }
            }
        }
    }
}