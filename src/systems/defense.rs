use crate::{
    components::{DefenseTag, HealthComponent},
    events::{DefenseItemGetEvent, EnemyReachedBottomEvent},
};
use amethyst::{
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
    ecs::*,
    shrev::{EventChannel, ReaderId},
};

#[derive(Default)]
pub struct DefenseSystem {
    defense_item_get_event_reader: Option<ReaderId<DefenseItemGetEvent>>,
    enemy_reached_bottom_event_reader: Option<ReaderId<EnemyReachedBottomEvent>>,
}

impl<'s> System<'s> for DefenseSystem {
    type SystemData = (
        Read<'s, EventChannel<DefenseItemGetEvent>>,
        Read<'s, EventChannel<EnemyReachedBottomEvent>>,
        ReadStorage<'s, DefenseTag>,
        WriteStorage<'s, HealthComponent>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.defense_item_get_event_reader = Some(
            world
                .fetch_mut::<EventChannel<DefenseItemGetEvent>>()
                .register_reader(),
        );
        self.enemy_reached_bottom_event_reader = Some(
            world
                .fetch_mut::<EventChannel<EnemyReachedBottomEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (
            defense_item_get_event_channel,
            enemy_reached_bottom_event_channel,
            defense_tags,
            mut healths,
        ): Self::SystemData,
    ) {
        for (_defense_tag, health) in (&defense_tags, &mut healths).join() {
            health.constrain();
        }

        for event in defense_item_get_event_channel
            .read(self.defense_item_get_event_reader.as_mut().unwrap())
        {
            for (_defense_tag, health) in (&defense_tags, &mut healths).join() {
                if event.stat_effects.contains_key("max_defense") {
                    health.max_health += event.stat_effects["max_defense"];
                    health.health += event.stat_effects["max_defense"];
                }
            }
        }

        for event in enemy_reached_bottom_event_channel
            .read(self.enemy_reached_bottom_event_reader.as_mut().unwrap())
        {
            for (_defense_tag, health) in (&defense_tags, &mut healths).join() {
                health.health -= event.damage;
            }
        }
    }
}
