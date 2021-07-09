use crate::{
    misc::{DefenseResource, HealthComponent},
    player::{BarrelRollAbilityComponent, PlayerComponent},
    store::StoreResource,
    visual::{spawn_status_unit, SpriteSheetsResource, StatusBarComponent, StatusType},
};
use amethyst::ecs::prelude::{
    Entities, Join, LazyUpdate, ReadExpect, ReadStorage, System, WriteStorage,
};

const HEALTH_SPRITE_INDEX: usize = 0;
const DEFENSE_SPRITE_INDEX: usize = 1;
const ROLL_SPRITE_INDEX: usize = 2;
const RESTOCK_SPRITE_INDEX: usize = 3;

/// Handles status bars
pub struct StatusBarSystem;

impl<'s> System<'s> for StatusBarSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, StatusBarComponent>,
        ReadStorage<'s, PlayerComponent>,
        ReadStorage<'s, BarrelRollAbilityComponent>,
        ReadStorage<'s, HealthComponent>,
        ReadExpect<'s, StoreResource>,
        ReadExpect<'s, SpriteSheetsResource>,
        ReadExpect<'s, DefenseResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut status_bars,
            players,
            barrel_roll_abilities,
            healths,
            store_resource,
            sprite_resource,
            defense_resource,
            lazy_update,
        ): Self::SystemData,
    ) {
        for status_bar in (&mut status_bars).join() {
            match status_bar.status_type {
                StatusType::Health => {
                    for (_player, health) in (&players, &healths).join() {
                        if let Some(status_position) = status_bar.update_units_y(
                            health.health.get_max_health(),
                            health.health.get_health(),
                            &entities,
                        ) {
                            status_bar.status_unit_stack.push(spawn_status_unit(
                                &entities,
                                &sprite_resource,
                                HEALTH_SPRITE_INDEX,
                                status_position,
                                &lazy_update,
                            ));
                        }
                    }
                }

                StatusType::Defense => {
                    if let Some(status_position) = status_bar.update_units_y(
                        defense_resource.defense.get_max_health(),
                        defense_resource.defense.get_health(),
                        &entities,
                    ) {
                        status_bar.status_unit_stack.push(spawn_status_unit(
                            &entities,
                            &sprite_resource,
                            DEFENSE_SPRITE_INDEX,
                            status_position,
                            &lazy_update,
                        ));
                    }
                }

                StatusType::Roll => {
                    for barrel_roll_ability in (&barrel_roll_abilities).join() {
                        if !barrel_roll_ability.is_ready() {
                            if let Some(status_position) = status_bar.update_units_x(
                                barrel_roll_ability.execution_timer.get_period(),
                                barrel_roll_ability.execution_timer.get_time_left(),
                                &entities,
                            ) {
                                status_bar.status_unit_stack.push(spawn_status_unit(
                                    &entities,
                                    &sprite_resource,
                                    ROLL_SPRITE_INDEX,
                                    status_position,
                                    &lazy_update,
                                ));
                            }
                        }
                    }
                }

                StatusType::Restock => {
                    if let Some(status_position) = status_bar.update_units_x(
                        store_resource.timer.get_period(),
                        store_resource.timer.get_time_left(),
                        &entities,
                    ) {
                        status_bar.status_unit_stack.push(spawn_status_unit(
                            &entities,
                            &sprite_resource,
                            RESTOCK_SPRITE_INDEX,
                            status_position,
                            &lazy_update,
                        ));
                    }
                }
            }
        }
    }
}
