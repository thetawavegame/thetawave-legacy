use amethyst::{
    core::{
        nalgebra::Vector3,
    },
    ecs::prelude::{Entities, Join, System, WriteStorage, ReadStorage, ReadExpect, LazyUpdate},
};

use crate::{
    components::{Spaceship, RollBar},
    entities::spawn_roll_unit,
    resources::SpriteResource,
};


pub struct RollBarSystem;
impl<'s> System<'s> for RollBarSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, RollBar>,
        ReadStorage<'s, Spaceship>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut roll_bars, spaceships, sprite_resource, lazy_update): Self::SystemData) {
        for roll_bar in (&mut roll_bars).join() {

            for spaceship in (&spaceships).join() {
                let roll_divisor = spaceship.barrel_cooldown/28.0;
                let mut roll_unit_num = ((spaceship.barrel_cooldown - spaceship.barrel_reset_timer) / roll_divisor).ceil() as usize;

                //push units on
                if roll_unit_num > roll_bar.roll_stack.len() {
                    let roll_position = Vector3::new(
                        roll_bar.x_pos, roll_bar.y_pos, 0.9,
                    );
                    roll_bar.x_pos += 1.0;
                    roll_bar.roll_stack.push(spawn_roll_unit(&entities, &sprite_resource, 12, roll_position, &lazy_update));
                }

                if roll_unit_num < roll_bar.roll_stack.len() {
                    if let Some(unit) = roll_bar.roll_stack.pop() {
                        roll_bar.x_pos -= 1.0;
                        let _result = entities.delete(unit);
                    }

                }
            }
        }
    }
}