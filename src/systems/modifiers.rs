use crate::{
    components::{
        BarrelRollAbilityComponent, BlasterComponent, HealthComponent, ManualFireComponent,
        Motion2DComponent, PlayerComponent,
    },
    entities::SpawnableType,
    events::{ConsumableGetEvent, ItemGetEvent},
    resources::{ConsumableModifiersResource, DefenseResource, ItemModifiersResource, Modifier},
};
use amethyst::{
    ecs::*,
    ecs::{Read, ReadExpect, System, WriteStorage},
    shrev::EventChannel,
};

#[derive(Default)]
pub struct ModifiersSystem {
    item_get_event_reader: Option<ReaderId<ItemGetEvent>>,
    consumable_get_event_reader: Option<ReaderId<ConsumableGetEvent>>,
}

impl<'s> System<'s> for ModifiersSystem {
    type SystemData = (
        Read<'s, EventChannel<ItemGetEvent>>,
        Read<'s, EventChannel<ConsumableGetEvent>>,
        WriteStorage<'s, BarrelRollAbilityComponent>,
        WriteStorage<'s, HealthComponent>,
        WriteStorage<'s, BlasterComponent>,
        WriteStorage<'s, ManualFireComponent>,
        WriteStorage<'s, Motion2DComponent>,
        WriteStorage<'s, PlayerComponent>,
        ReadExpect<'s, ItemModifiersResource>,
        ReadExpect<'s, ConsumableModifiersResource>,
        WriteExpect<'s, DefenseResource>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.item_get_event_reader = Some(
            world
                .fetch_mut::<EventChannel<ItemGetEvent>>()
                .register_reader(),
        );
        self.consumable_get_event_reader = Some(
            world
                .fetch_mut::<EventChannel<ConsumableGetEvent>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (
            item_get_event_channel,
            consumable_get_event_channel,
            mut barrel_roll_ability_components,
            mut health_components,
            mut blaster_components,
            mut manual_fire_components,
            mut motion_2d_components,
            mut player_components,
            item_modifiers_resource,
            consumable_modifiers_resource,
            mut defense_resource,
        ): Self::SystemData,
    ) {
        for event in item_get_event_channel.read(self.item_get_event_reader.as_mut().unwrap()) {
            apply_modifiers(
                &item_modifiers_resource[&event.item_type],
                event.player_entity,
                SpawnableType::Item(event.item_type.clone()),
                &mut barrel_roll_ability_components,
                &mut health_components,
                &mut blaster_components,
                &mut manual_fire_components,
                &mut motion_2d_components,
                &mut player_components,
                &mut defense_resource,
            )
        }

        for event in
            consumable_get_event_channel.read(self.consumable_get_event_reader.as_mut().unwrap())
        {
            apply_modifiers(
                &consumable_modifiers_resource[&event.consumable_type],
                event.player_entity,
                SpawnableType::Consumable(event.consumable_type.clone()),
                &mut barrel_roll_ability_components,
                &mut health_components,
                &mut blaster_components,
                &mut manual_fire_components,
                &mut motion_2d_components,
                &mut player_components,
                &mut defense_resource,
            )
        }

        // TODO: remove item effects for item remove event
        // for event in item_remove_event_channel...
    }
}

pub fn apply_modifiers(
    modifiers: &Vec<Modifier>,
    player_entity: Entity,
    spawnable_type: SpawnableType,
    barrel_roll_ability_components: &mut WriteStorage<BarrelRollAbilityComponent>,
    health_components: &mut WriteStorage<HealthComponent>,
    blaster_components: &mut WriteStorage<BlasterComponent>,
    manual_fire_components: &mut WriteStorage<ManualFireComponent>,
    motion_2d_components: &mut WriteStorage<Motion2DComponent>,
    player_components: &mut WriteStorage<PlayerComponent>,
    defense_resource: &mut WriteExpect<DefenseResource>,
) {
    let player_barrel_roll_ability = barrel_roll_ability_components
        .get_mut(player_entity)
        .unwrap();
    let player_health = health_components.get_mut(player_entity).unwrap();
    let player_blaster = blaster_components.get_mut(player_entity).unwrap();
    let player_manual_fire = manual_fire_components.get_mut(player_entity).unwrap();
    let player_motion2d = motion_2d_components.get_mut(player_entity).unwrap();
    let player_component = player_components.get_mut(player_entity).unwrap();

    match spawnable_type {
        SpawnableType::Item(item_type) => {
            player_component.items.push(item_type);
        }

        SpawnableType::Consumable(_consumable_type) => {}

        _ => {
            panic!("only consumables and items can have modifiers")
        }
    }

    for modifier in modifiers.iter() {
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
            }

            Modifier::Health(val) => {
                player_health.value += val;
            }

            Modifier::ProjectileSize(val) => {
                player_blaster.size_multiplier += val;
            }

            Modifier::MaximumDefense(val) => {
                defense_resource.max_defense += val;
            }

            Modifier::Defense(val) => {
                defense_resource.value += val;
            }

            Modifier::Armor(val) => {
                player_health.armor += val;
            }

            Modifier::Money(val) => {
                player_component.money += val;
            }
        }
    }
}
