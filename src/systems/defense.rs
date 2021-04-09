use crate::{
    components::{DefenseTag, HealthComponent},
    events::{ItemGetEvent, MobReachedBottomEvent},
};
use amethyst::{
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
    ecs::*,
    shrev::{EventChannel, ReaderId},
};

#[derive(Default)]
pub struct DefenseSystem {
    item_get_event_reader: Option<ReaderId<ItemGetEvent>>,
    mob_reached_bottom_event_reader: Option<ReaderId<MobReachedBottomEvent>>,
}

impl<'s> System<'s> for DefenseSystem {
    type SystemData = (
        Read<'s, EventChannel<ItemGetEvent>>,
        Read<'s, EventChannel<MobReachedBottomEvent>>,
        ReadStorage<'s, DefenseTag>,
        WriteStorage<'s, HealthComponent>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.item_get_event_reader = Some(
            world
                .fetch_mut::<EventChannel<ItemGetEvent>>()
                .register_reader(),
        );
        self.mob_reached_bottom_event_reader = Some(
            world
                .fetch_mut::<EventChannel<MobReachedBottomEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (
            item_get_event_channel,
            mob_reached_bottom_event_channel,
            defense_tags,
            mut healths,
        ): Self::SystemData,
    ) {
        for (_defense_tag, health) in (&defense_tags, &mut healths).join() {
            health.constrain();
        }

        for event in item_get_event_channel.read(self.item_get_event_reader.as_mut().unwrap()) {
            for (_defense_tag, health) in (&defense_tags, &mut healths).join() {
                if event.stat_effects.contains_key("max_defense") {
                    health.max_value += event.stat_effects["max_defense"];
                    health.value += event.stat_effects["max_defense"];
                }
            }
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
