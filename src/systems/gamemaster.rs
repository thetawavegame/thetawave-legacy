use crate::components::{GameMaster, PhaseType};
use amethyst::{
    core::timing::Time,
    ecs::prelude::{Join, Read, System, WriteStorage},
};

pub struct GameMasterSystem;

impl<'s> System<'s> for GameMasterSystem {
    type SystemData = (WriteStorage<'s, GameMaster>, Read<'s, Time>);

    fn run(&mut self, (mut gamemasters, time): Self::SystemData) {
        for gamemaster in (&mut gamemasters).join() {
            match gamemaster.phase_map[gamemaster.phase_idx].phase_type {
                PhaseType::Invasion => {
                    gamemaster.iterate_tick(time.delta_seconds());
                }

                PhaseType::Rest => {
                    gamemaster.iterate_tick(time.delta_seconds());
                }

                PhaseType::Boss => {}
            }
        }
    }
}
