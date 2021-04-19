use crate::{
    components::{
        BarrelRollAbilityComponent, BlasterComponent, HealthComponent, ManualFireComponent,
        Motion2DComponent, PlayerComponent,
    },
    events::ItemGetEvent,
    resources::{DefenseResource, ItemModifiersResource, Modifier},
};
use amethyst::{
    ecs::*,
    ecs::{Read, ReadExpect, System, WriteStorage},
    shrev::EventChannel,
};

#[derive(Default)]
pub struct ModifiersSystem {
    item_get_event_reader: Option<ReaderId<ItemGetEvent>>,
}

impl<'s> System<'s> for ModifiersSystem {
    type SystemData = (
        Read<'s, EventChannel<ItemGetEvent>>,
        WriteStorage<'s, BarrelRollAbilityComponent>,
        WriteStorage<'s, HealthComponent>,
        WriteStorage<'s, BlasterComponent>,
        WriteStorage<'s, ManualFireComponent>,
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, PlayerComponent>,
        ReadExpect<'s, ItemModifiersResource>,
        WriteExpect<'s, DefenseResource>,
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
            item_get_event_channel,
            mut barrel_roll_abilities,
            mut healths,
            mut blasters,
            mut manual_fires,
            mut motion2ds,
            mut player_components,
            item_modifiers_resource,
            mut defense_resource,
        ): Self::SystemData,
    ) {
        for event in item_get_event_channel.read(self.item_get_event_reader.as_mut().unwrap()) {
            let player_barrel_roll_ability =
                barrel_roll_abilities.get_mut(event.player_entity).unwrap();
            let player_health = healths.get_mut(event.player_entity).unwrap();
            let player_blaster = blasters.get_mut(event.player_entity).unwrap();
            let player_manual_fire = manual_fires.get_mut(event.player_entity).unwrap();
            let player_motion2d = motion2ds.get_mut(event.player_entity).unwrap();
            let player_component = player_components.get_mut(event.player_entity).unwrap();

            // add item to players item
            player_component.items.push(event.item_type.clone());

            for modifier in item_modifiers_resource[&event.item_type].iter() {
                match modifier {
                    Modifier::BarrelImmunity(val) => {
                        player_barrel_roll_ability.steel_barrel = *val;
                    }

                    Modifier::ProjectileCount(val) => {
                        player_blaster.count += val;
                    }

                    Modifier::ProjectileFirePeriod(val) => {
                        player_manual_fire.period += val;
                    }

                    Modifier::ProjectileDamage(val) => {
                        player_blaster.damage += val;
                    }

                    Modifier::MaximumSpeed(val) => {
                        player_motion2d.max_speed.x += val;
                        player_motion2d.max_speed.y += val;
                    }

                    Modifier::CriticalDamageChance(val) => {
                        player_blaster.crit_chance += val;
                    }

                    Modifier::PoisonChance(val) => {
                        player_blaster.poison_chance += val;
                    }

                    Modifier::AbilityCooldown(val) => {
                        player_barrel_roll_ability.execute_cooldown += val;
                    }

                    Modifier::Acceleration(val) => {
                        player_motion2d.acceleration.x += val;
                        player_motion2d.acceleration.y += val;
                    }

                    Modifier::Deceleration(val) => {
                        player_motion2d.deceleration.x += val;
                        player_motion2d.deceleration.y += val;
                    }

                    Modifier::MaximumHealth(val) => {
                        player_health.max_value += val;
                        player_health.value += val;
                    }

                    Modifier::ProjectileSize(val) => {
                        player_blaster.size_multiplier += val;
                    }

                    Modifier::MaximumDefense(val) => {
                        defense_resource.max_defense += val;
                        defense_resource.value += val;
                    }
                }
            }
        }

        // TODO: remove item effects for item remove event
        // for event in item_remove_event_channel...
    }
}
