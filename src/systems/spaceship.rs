use crate::{
    audio::Sounds,
    components::{
        BlasterComponent, HealthComponent, ManualFireComponent, Motion2DComponent, Spaceship,
    },
    events::{ItemGetEvent, PlayAudioEvent},
    resources::SpriteResource,
};
use amethyst::{
    core::{timing::Time, Transform},
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
        WriteStorage<'s, Spaceship>,
        WriteStorage<'s, HealthComponent>,
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, BlasterComponent>,
        WriteStorage<'s, ManualFireComponent>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        ReadExpect<'s, SpriteResource>,
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
            mut spaceships,
            mut healths,
            mut motion2ds,
            mut blasters,
            mut manual_fires,
            input,
            time,
            sprite_resource,
            lazy_update,
            item_get_event_channel,
            mut play_audio_channel,
            sounds,
        ): Self::SystemData,
    ) {
        // collect input bools
        let shoot_action = input.action_is_down("shoot").unwrap();
        let mut barrel_left = input.action_is_down("barrel_left").unwrap();
        let mut barrel_right = input.action_is_down("barrel_right").unwrap();

        for (spaceship, health, transform, motion2d, blaster, manual_fire) in (
            &mut spaceships,
            &mut healths,
            &mut transforms,
            &mut motion2ds,
            &blasters,
            &mut manual_fires,
        )
            .join()
        {
            // update pos_x and pos_y variables of spaceship
            spaceship.update_location(transform.translation().x, transform.translation().y);

            // barrel roll input cooldown
            // amount of time until new barrel roll can be initiated
            if spaceship.barrel_input_cooldown(time.delta_seconds()) {
                barrel_left = false;
                barrel_right = false;
            }

            //barrel roll action cooldown
            //amount of time until barrel roll is complete
            if spaceship.barrel_action_cooldown(time.delta_seconds(), motion2d) {
                barrel_left = false;
                barrel_right = false;
            }

            if !spaceship.barrel_action_left
                && !spaceship.barrel_action_right
                && shoot_action
                && manual_fire.ready
            {
                blaster.fire(
                    motion2d,
                    transform,
                    &entities,
                    &sprite_resource,
                    &lazy_update,
                );
                manual_fire.ready = false;
                play_audio_channel.single_write(PlayAudioEvent {
                    source: sounds.spaceship_laser_sfx.clone(),
                });
            }

            spaceship.initiate_barrel_roll(barrel_left, barrel_right);
            health.constrain();
        }

        for event in item_get_event_channel.read(self.item_get_event_reader.as_mut().unwrap()) {
            let spaceship = spaceships.get_mut(event.player_entity).unwrap();
            let spaceship_health = healths.get_mut(event.player_entity).unwrap();
            let blaster = blasters.get_mut(event.player_entity).unwrap();
            let manual_fire = manual_fires.get_mut(event.player_entity).unwrap();
            let motion = motion2ds.get_mut(event.player_entity).unwrap();

            if event.bool_effects.contains_key("barrel_immunity") {
                spaceship.steel_barrel = event.bool_effects["barrel_immunity"];
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
                spaceship.barrel_cooldown += event.stat_effects["barrel_cooldown"];
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
