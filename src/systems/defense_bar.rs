use amethyst::{
    core::{
        nalgebra::Vector3,
    },
    ecs::prelude::{Entities, Join, System, WriteStorage, ReadExpect, LazyUpdate},
};

use crate::{
    components::{DefenseBar, Defense},
    entities::spawn_defense_unit,
    resources::SpriteResource,
};



pub struct DefenseBarSystem;
impl<'s> System<'s> for DefenseBarSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, DefenseBar>,
        WriteStorage<'s, Defense>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut defense_bars, mut defenses, sprite_resource, lazy_update): Self::SystemData) {
        for defense_bar in (&mut defense_bars).join() {
            for defense in (&mut defenses).join() {

                if defense.defense < 0.0 {
                    defense.defense = 0.0;
                } else if defense.defense > defense.max_defense {
                    defense.defense = defense.max_defense;
                }

                //find the number of points per unit in the bar and the number of units needed
                let defense_divisor = defense.max_defense / 63.0;
                let defense_unit_num = ((defense.defense) / defense_divisor).ceil() as usize;


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
                        let _result = entities.delete(unit);
                        defense_bar.y_pos -= 1.0;
                    }
                }
            }
        }
    }
}