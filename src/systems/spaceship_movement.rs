use amethyst::{
    core::{
        Transform,
        timing::Time,
    },
    ecs::{Join, Read, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::{
    components::{Spaceship, Rigidbody},
    
};

use crate::space_shooter::{ARENA_MIN_X, ARENA_MAX_X, ARENA_MIN_Y, ARENA_MAX_Y};


pub struct SpaceshipMovementSystem;
impl<'s> System<'s> for SpaceshipMovementSystem {

    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Spaceship>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut spaceships, input, time): Self::SystemData) {

        let x_move = input.axis_value("player_x").unwrap() as f32;
        let y_move = input.axis_value("player_y").unwrap() as f32;

        for (spaceship, transform) in (&mut spaceships, &mut transforms).join() {

            //keep spaceship with bounds of arena
            spaceship.constrain_to_arena(transform);

            //if the spaceships speed is greater than knockback max speed, immediately set velocity to knockback_max_speed
            spaceship.limit_knockback();

            //if the spaceships speed is greater than its max speed, decelerate to equal max_speed
            spaceship.limit_speed();

            //if barrel rolling a direction use the barrel roll x velocity, otherwise accelerate normally
            if spaceship.barrel_action_left {
                spaceship.set_current_velocity_x(-1.0 * spaceship.barrel_speed);
            } else if spaceship.barrel_action_right {
                spaceship.set_current_velocity_x(spaceship.barrel_speed);
            } else {
                spaceship.accelerate(x_move, y_move);
            }

            spaceship.update_position(transform, time.delta_seconds());
        }
    }
}