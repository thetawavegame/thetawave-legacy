use crate::{
    events::PlayerCollisionEvent,
    misc::components::BarrierComponent,
    motion::Motion2DComponent,
    player::components::{AbilityDirection, BarrelRollAbilityComponent, CooldownAbility},
    spawnable::MobComponent,
};
use amethyst::{
    core::timing::Time,
    ecs::*,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
    shrev::{EventChannel, ReaderId},
};

/// Manages barrel roll ability
#[derive(Default)]
pub struct BarrelRollAbilitySystem {
    event_reader: Option<ReaderId<PlayerCollisionEvent>>,
}

impl<'s> System<'s> for BarrelRollAbilitySystem {
    type SystemData = (
        Read<'s, EventChannel<PlayerCollisionEvent>>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        WriteStorage<'s, BarrelRollAbilityComponent>,
        WriteStorage<'s, Motion2DComponent>,
        ReadStorage<'s, MobComponent>,
        ReadStorage<'s, BarrierComponent>,
    );

    fn run(
        &mut self,
        (
            collision_event_channel,
            input,
            time,
            mut barrel_roll_abilities,
            mut motion2ds,
            mobs,
            barriers,
        ): Self::SystemData,
    ) {
        for (barrel_roll_ability, motion2d) in (&mut barrel_roll_abilities, &mut motion2ds).join() {
            // execute barrel roll on input down
            barrel_roll_ability.execute_action(&input);

            // update ability and timers
            barrel_roll_ability.update(time.delta_seconds());

            // change direction if colliding with mob
            for event in collision_event_channel.read(self.event_reader.as_mut().unwrap()) {
                if let Some(_mob) = mobs.get(event.colliding_entity) {
                    barrel_roll_ability.invert_direction();
                } else if let Some(_barrier) = barriers.get(event.colliding_entity) {
                    barrel_roll_ability.invert_direction();
                }
            }

            // change velocity if barrel rolling
            match barrel_roll_ability.get_direction() {
                AbilityDirection::Left => {
                    motion2d.velocity.x = -barrel_roll_ability.get_speed();
                }
                AbilityDirection::Right => {
                    motion2d.velocity.x = barrel_roll_ability.get_speed();
                }
                AbilityDirection::None => {}
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<PlayerCollisionEvent>>()
                .register_reader(),
        );
    }
}
