use crate::{
    components::{DefenseTag, HealthComponent},
    events::DefenseItemGetEvent,
};
use amethyst::{
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
    ecs::*,
    shrev::{EventChannel, ReaderId},
};

#[derive(Default)]
pub struct DefenseSystem {
    event_reader: Option<ReaderId<DefenseItemGetEvent>>,
}

impl<'s> System<'s> for DefenseSystem {
    type SystemData = (
        Read<'s, EventChannel<DefenseItemGetEvent>>,
        ReadStorage<'s, DefenseTag>,
        WriteStorage<'s, HealthComponent>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<DefenseItemGetEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (defense_item_get_event_channel, defense_tags, mut healths): Self::SystemData,
    ) {
        for (defense_tag, health) in (&defense_tags, &mut healths).join() {
            health.constrain();
        }

        for event in defense_item_get_event_channel.read(self.event_reader.as_mut().unwrap()) {
            for (defense_tag, health) in (&defense_tags, &mut healths).join() {
                if event.stat_effects.contains_key("max_defense") {
                    health.max_health += event.stat_effects["max_defense"];
                    health.health += event.stat_effects["max_defense"];
                }
            }
        }
    }
}
