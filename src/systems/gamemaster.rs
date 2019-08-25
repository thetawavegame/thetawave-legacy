use amethyst::{
    core::{
        transform::Transform,
        timing::Time,
    },
    ecs::prelude::{Entities,Join, ReadStorage, System, Read, WriteStorage},
};

use crate::{
    components::{GameMaster},
};


pub struct GameMasterSystem;
impl<'s> System<'s> for GameMasterSystem {

    type SystemData = (
        WriteStorage<'s, GameMaster>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut gamemasters, time): Self::SystemData) {
        for gamemaster in (&mut gamemasters).join() {
            if gamemaster.tick_timer > 0.0 {
                gamemaster.tick_timer -= time.delta_seconds();
            } else if gamemaster.phase_idx < gamemaster.last_phase {
                println!("phase index: {}\tcurrent_tick: {}", gamemaster.phase_idx, gamemaster.current_tick);
                gamemaster.tick_timer = gamemaster.tick_length;
                gamemaster.current_tick += 1;
                if gamemaster.current_tick >= gamemaster.phase_map[gamemaster.phase_idx].length {
                    gamemaster.phase_idx += 1;
                    gamemaster.current_tick = 0;
                }
            } else {
                println!("phase index: {}\tcurrent_tick: {}", gamemaster.phase_idx, gamemaster.current_tick);
                gamemaster.current_tick += 1;
            }
        }
    }
}
