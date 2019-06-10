use amethyst::{
    core::{
        nalgebra::Vector3,
    },
    ecs::prelude::{Entities, Join, System, WriteStorage, ReadExpect, LazyUpdate},
};

use crate::{
    components::{DefenseBar},
    entities::spawn_defense_unit,
    resources::SpriteResource,
};



pub struct DefenseBarSystem;
impl<'s> System<'s> for DefenseBarSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, DefenseBar>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut defense_bars, sprite_resource, lazy_update): Self::SystemData) {
        for defense_bar in (&mut defense_bars).join() {

            //find the number of points per unit in the bar and the number of units needed
            let defense_divisor = defense_bar.max_defense/63.0;
            let mut defense_unit_num = ((defense_bar.defense) / defense_divisor).ceil() as usize;
            if defense_bar.defense <= 0.0 {
                defense_unit_num = 0;
            }

            //push units onto stack if needed
            if defense_unit_num > defense_bar.defense_stack.len() {
                let defense_position = Vector3::new(
                    defense_bar.x_pos, defense_bar.y_pos, 0.9,
                );
                defense_bar.y_pos += 1.0;
                defense_bar.defense_stack.push(spawn_defense_unit(&entities, &sprite_resource, 11, defense_position, &lazy_update));
            }

            //delete units from stack if needed
            if defense_unit_num < defense_bar.defense_stack.len() {
                if let Some(unit) = defense_bar.defense_stack.pop() {
                    entities.delete(unit);
                }

            }
        }
    }
}