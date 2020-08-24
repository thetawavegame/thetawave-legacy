use amethyst::{
    ecs::{World, WorldExt},
    prelude::Builder,
};

use crate::{
    components::{BossType, GameMaster, Phase, PhaseType},
    constants::{LAST_PHASE_IDX, STARTING_PHASE_IDX, STARTING_TICK, TICK_LENGTH},
};

pub fn initialize_gamemaster(world: &mut World) {
    let mut phase_map: Vec<Phase> = Vec::new();

    let phase_0 = Phase {
        phase_type: PhaseType::Rest,
        boss_type: BossType::None,
        length: 8,
        boss_spawned: false,
    };

    let phase_1 = Phase {
        phase_type: PhaseType::Invasion,
        boss_type: BossType::Repeater,
        length: 5,
        boss_spawned: false,
    };

    let phase_2 = Phase {
        phase_type: PhaseType::Rest,
        boss_type: BossType::None,
        length: 4,
        boss_spawned: false,
    };

    let phase_3 = Phase {
        phase_type: PhaseType::Boss,
        boss_type: BossType::Repeater,
        length: 0,
        boss_spawned: false,
    };

    let phase_4 = Phase {
        phase_type: PhaseType::Rest,
        boss_type: BossType::None,
        length: 200,
        boss_spawned: false,
    };

    //phase_map.push(phase_0);
    //phase_map.push(phase_1);
    //phase_map.push(phase_2);
    phase_map.push(phase_3);
    phase_map.push(phase_4);

    world
        .create_entity()
        .with(GameMaster {
            phase_map,
            phase_idx: STARTING_PHASE_IDX,
            last_phase: LAST_PHASE_IDX,
            current_tick: STARTING_TICK,
            tick_timer: TICK_LENGTH,
            tick_length: TICK_LENGTH,
        })
        .build();
}
