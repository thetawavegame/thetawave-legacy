use crate::resources::{PhaseManagerResource, PhaseType};
use amethyst::{
    core::timing::Time,
    ecs::prelude::{Read, System, Write},
};

pub struct GameMasterSystem;

impl<'s> System<'s> for GameMasterSystem {
    type SystemData = (Write<'s, PhaseManagerResource>, Read<'s, Time>);

    fn run(&mut self, (mut phase_manager, time): Self::SystemData) {
        match phase_manager.phase_map[phase_manager.phase_idx].phase_type {
            PhaseType::Invasion => {
                phase_manager.update(time.delta_seconds());
            }

            PhaseType::Rest => {
                phase_manager.update(time.delta_seconds());
            }

            PhaseType::Boss => {}
        }
    }
}
