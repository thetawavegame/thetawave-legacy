use amethyst::{
    prelude::Builder,
    ecs::prelude::World,
};
use crate::{
    components::{GameMaster, Phase, PhaseType},
};

const STARTING_TICK: usize = 0;
const STARTING_PHASE_IDX: usize = 0;
const LAST_PHASE_IDX: usize = 4;
const TICK_LENGTH: f32 = 1.0;

pub fn initialise_gamemaster(world: &mut World) {

    let mut phase_map: Vec<Phase> = vec![];

    let phase_1 = Phase {
        phase_type: PhaseType::Rest,
        length: 8, 
    };

    let phase_2 = Phase {
        phase_type: PhaseType::Invasion,
        length: 300,
    };

    let phase_3 = Phase {
        phase_type: PhaseType::Rest,
        length: 60,
    };

    let phase_4 = Phase {
        phase_type: PhaseType::Invasion,
        length: 300,
    };
    
    phase_map.push(phase_1);
    phase_map.push(phase_2);
    phase_map.push(phase_3);
    phase_map.push(phase_4);

    world
        .create_entity()
        .with(GameMaster {
            phase_map: phase_map,
            phase_idx: STARTING_PHASE_IDX, 
            last_phase: LAST_PHASE_IDX,
            current_tick: STARTING_TICK,
            tick_timer: TICK_LENGTH,
            tick_length: TICK_LENGTH,
        })
        .build();
}