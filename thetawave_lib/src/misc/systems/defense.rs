use crate::{events::MobReachedBottomEvent, misc::DefenseResource};
use amethyst::{
    ecs::prelude::{System, WriteExpect},
    ecs::*,
    shrev::{EventChannel, ReaderId},
};

/// Handles defense objective
#[derive(Default)]
pub struct DefenseSystem {
    mob_reached_bottom_event_reader: Option<ReaderId<MobReachedBottomEvent>>,
}

impl<'s> System<'s> for DefenseSystem {
    type SystemData = (
        Read<'s, EventChannel<MobReachedBottomEvent>>,
        WriteExpect<'s, DefenseResource>,
    );

    fn run(&mut self, (mob_reached_bottom_event_channel, mut defense_resource): Self::SystemData) {
        for event in mob_reached_bottom_event_channel
            .read(self.mob_reached_bottom_event_reader.as_mut().unwrap())
        {
            defense_resource.defense.take_damage(event.damage);
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.mob_reached_bottom_event_reader = Some(
            world
                .fetch_mut::<EventChannel<MobReachedBottomEvent>>()
                .register_reader(),
        );
    }
}
