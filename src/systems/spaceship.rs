use crate::{
    audio::Sounds,
    components::{
        BarrelRollAbilityComponent, BlasterComponent, HealthComponent, ManualFireComponent,
        Motion2DComponent,
    },
    events::{ItemGetEvent, PlayAudioEvent},
    resources::SpriteSheetsResource,
};
use amethyst::{
    core::Transform,
    ecs::*,
    ecs::{Entities, Join, LazyUpdate, Read, ReadExpect, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    shrev::EventChannel,
};

#[derive(Default)]
pub struct SpaceshipSystem {
    item_get_event_reader: Option<ReaderId<ItemGetEvent>>,
}

impl<'s> System<'s> for SpaceshipSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, BarrelRollAbilityComponent>,
        WriteStorage<'s, HealthComponent>,
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, BlasterComponent>,
        WriteStorage<'s, ManualFireComponent>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, SpriteSheetsResource>,
        ReadExpect<'s, LazyUpdate>,
        Read<'s, EventChannel<ItemGetEvent>>,
        Write<'s, EventChannel<PlayAudioEvent>>,
        ReadExpect<'s, Sounds>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.item_get_event_reader = Some(
            world
                .fetch_mut::<EventChannel<ItemGetEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (
            entities,
            mut transforms,
            mut barrel_roll_abilities,
            mut healths,
            mut motion2ds,
            mut blasters,
            mut manual_fires,
            input,
            sprite_resource,
            lazy_update,
            item_get_event_channel,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        // collect input bools
        let shoot_action = input.action_is_down("shoot").unwrap();

        for (health, transform, motion2d, blaster, manual_fire) in (
            &mut healths,
            &mut transforms,
            &mut motion2ds,
            &blasters,
            &mut manual_fires,
        )
            .join()
        {
            if shoot_action && manual_fire.ready {
                blaster.fire(
                    motion2d,
                    transform,
                    &entities,
                    &sprite_resource,
                    &lazy_update,
                );
                manual_fire.ready = false;
                play_audio_channel.single_write(PlayAudioEvent {
                    source: sounds.sound_effects["laser_blast"].clone(),
                });
            }

            health.constrain();
        }

        for event in item_get_event_channel.read(self.item_get_event_reader.as_mut().unwrap()) {
            let barrel_roll_ability = barrel_roll_abilities.get_mut(event.player_entity).unwrap();
            let spaceship_health = healths.get_mut(event.player_entity).unwrap();
            let blaster = blasters.get_mut(event.player_entity).unwrap();
            let manual_fire = manual_fires.get_mut(event.player_entity).unwrap();
            let motion = motion2ds.get_mut(event.player_entity).unwrap();

            if event.bool_effects.contains_key("barrel_immunity") {
                barrel_roll_ability.steel_barrel = event.bool_effects["barrel_immunity"];
            }

            if event.stat_effects.contains_key("blast_count") {
                blaster.count += event.stat_effects["blast_count"] as usize;
            }

            if event.stat_effects.contains_key("blast_fire_speed") {
                manual_fire.period += event.stat_effects["blast_fire_speed"];
            }

            if event.stat_effects.contains_key("blast_damage") {
                blaster.damage += event.stat_effects["blast_damage"];
            }

            if event.stat_effects.contains_key("max_speed") {
                motion.max_speed.x += event.stat_effects["max_speed"];
                motion.max_speed.y += event.stat_effects["max_speed"];
            }
            if event.stat_effects.contains_key("crit_chance") {
                blaster.crit_chance += event.stat_effects["crit_chance"];
            }

            if event.stat_effects.contains_key("poison_chance") {
                blaster.poison_chance += event.stat_effects["poison_chance"];
            }

            if event.stat_effects.contains_key("barrel_cooldown") {
                barrel_roll_ability.execute_cooldown += event.stat_effects["barrel_cooldown"];
            }

            if event.stat_effects.contains_key("acceleration") {
                motion.acceleration.x += event.stat_effects["acceleration"];
                motion.acceleration.y += event.stat_effects["acceleration"];
            }

            if event.stat_effects.contains_key("deceleration") {
                motion.deceleration.x += event.stat_effects["deceleration"];
                motion.deceleration.y += event.stat_effects["deceleration"];
            }

            if event.stat_effects.contains_key("health_multiply") {
                spaceship_health.max_value *= event.stat_effects["health_multiply"];
                spaceship_health.value = spaceship_health.max_value;
            }

            if event.stat_effects.contains_key("health_add") {
                spaceship_health.max_value += event.stat_effects["health_add"];
                spaceship_health.value = spaceship_health.max_value;
            }

            if event.stat_effects.contains_key("blast_size") {
                blaster.size_multiplier += event.stat_effects["blast_size"];
            }
        }
    }
}
