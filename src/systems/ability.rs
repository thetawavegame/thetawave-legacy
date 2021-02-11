use crate::components::{BarrelRollAbilityComponent, CooldownAbility};
use amethyst::{
    core::timing::Time,
    ecs::{Join, Read, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

pub struct BarrelRollAbilitySystem;

impl<'s> System<'s> for BarrelRollAbilitySystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        WriteStorage<'s, BarrelRollAbilityComponent>,
    );

    fn run(&mut self, (input, time, mut barrel_roll_abilities): Self::SystemData) {
        for barrel_roll_ability in (&mut barrel_roll_abilities).join() {
            /*
            println!(
                "execute cooldown: {}\texecute timer: {}\taction_cooldown: {}\taction timer: {}\tability direction: {:?}",
                barrel_roll_ability.execute_cooldown,
                barrel_roll_ability.execute_timer,
                barrel_roll_ability.action_cooldown,
                barrel_roll_ability.action_timer,
                barrel_roll_ability.action_direction,
            );
            */

            // execute barrel roll on input down
            barrel_roll_ability.execute_action(&input);

            // update ability and timers
            barrel_roll_ability.update(time.delta_seconds());
        }
    }
}
