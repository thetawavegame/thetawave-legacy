use amethyst::{
    core::{
        timing::Time,
    },
    ecs::prelude::{Join, System, Read, WriteStorage},
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
            gamemaster.iterate_tick(time.delta_seconds());
        }
    }
}
