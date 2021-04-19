use crate::{
    components::{DefenseTag, HealthComponent},
    events::MobReachedBottomEvent,
};
use amethyst::{
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
    ecs::*,
    shrev::{EventChannel, ReaderId},
};

#[derive(Default)]
pub struct DefenseSystem {
    mob_reached_bottom_event_reader: Option<ReaderId<MobReachedBottomEvent>>,
}

impl<'s> System<'s> for DefenseSystem {
    type SystemData = (
        Read<'s, EventChannel<MobReachedBottomEvent>>,
        ReadStorage<'s, DefenseTag>,
        WriteStorage<'s, HealthComponent>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.mob_reached_bottom_event_reader = Some(
            world
                .fetch_mut::<EventChannel<MobReachedBottomEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (mob_reached_bottom_event_channel, defense_tags, mut healths): Self::SystemData,
    ) {
        for (_defense_tag, health) in (&defense_tags, &mut healths).join() {
            health.constrain();
        }

        for event in mob_reached_bottom_event_channel
            .read(self.mob_reached_bottom_event_reader.as_mut().unwrap())
        {
            for (_defense_tag, health) in (&defense_tags, &mut healths).join() {
                health.value -= event.damage;
            }
        }
    }
}
