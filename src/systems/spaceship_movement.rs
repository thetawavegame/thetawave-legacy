use amethyst::{
    core::{
        Transform,
        timing::Time,
    },
    ecs::{Join, Read, System, WriteStorage},
    input::InputHandler,
};

use crate::{
    components::{Spaceship},
};


pub struct SpaceshipMovementSystem;
impl<'s> System<'s> for SpaceshipMovementSystem {

    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Spaceship>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut spaceships, input, time): Self::SystemData) {

        let x_move = input.axis_value("player_x").unwrap() as f32;
        let y_move = input.axis_value("player_y").unwrap() as f32;

        for (spaceship, transform) in (&mut spaceships, &mut transforms).join() {

            let spaceship_x = transform.translation().x;
            let spaceship_y = transform.translation().y;

            //if barrel rolling a direction use the barrel roll x velocity, otherwise accelerate normally
            if spaceship.barrel_action_left {

                transform.set_x(spaceship_x - (spaceship.barrel_speed) * time.delta_seconds());
                transform.set_y(spaceship_y + (spaceship.current_velocity_y) * time.delta_seconds());

            } else if spaceship.barrel_action_right {

                transform.set_x(spaceship_x + (spaceship.barrel_speed) * time.delta_seconds());
                transform.set_y(spaceship_y + (spaceship.current_velocity_y) * time.delta_seconds());

            } else {
                //conditions for acceleration and deceleration in x
                if x_move > 0.0 && spaceship.current_velocity_x < spaceship.max_speed {
                    spaceship.current_velocity_x += spaceship.acceleration_x;
                } else if x_move < 0.0 && spaceship.current_velocity_x > (-1.0 * spaceship.max_speed) {
                    spaceship.current_velocity_x -= spaceship.acceleration_x;
                } else if x_move == 0.0 && spaceship.current_velocity_x > 0.0 {
                    spaceship.current_velocity_x -= spaceship.deceleration_x;
                } else if x_move == 0.0 && spaceship.current_velocity_x < 0.0 {
                    spaceship.current_velocity_x += spaceship.deceleration_x;
                }

                //conditions for acceleration and deceleration in y
                if y_move > 0.0 && spaceship.current_velocity_y < spaceship.max_speed {
                    spaceship.current_velocity_y += spaceship.acceleration_y;
                } else if y_move < 0.0 && spaceship.current_velocity_y > (-1.0 * spaceship.max_speed) {
                    spaceship.current_velocity_y -= spaceship.acceleration_y;
                } else if y_move == 0.0 && spaceship.current_velocity_y > 0.0 {
                    spaceship.current_velocity_y -= spaceship.deceleration_y;
                } else if y_move == 0.0 && spaceship.current_velocity_y < 0.0 {
                    spaceship.current_velocity_y += spaceship.deceleration_y;
                }

                //transform the spaceship in x and y by the currrent velocity in x and y
                transform.set_x(spaceship_x + (spaceship.current_velocity_x) * time.delta_seconds());
                transform.set_y(spaceship_y + (spaceship.current_velocity_y) * time.delta_seconds());
            }
        }
    }
}