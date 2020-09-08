use crate::components::{Motion2DComponent, Rigidbody, Spaceship};
use amethyst::{
    core::{timing::Time, Transform},
    ecs::{Join, Read, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

pub struct SpaceshipMovementSystem;

impl<'s> System<'s> for SpaceshipMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Spaceship>,
        WriteStorage<'s, Motion2DComponent>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (mut transforms, mut spaceships, mut motion_2d_components, input, time): Self::SystemData,
    ) {
        let x_move = input.axis_value("player_x").unwrap() as f32;
        let y_move = input.axis_value("player_y").unwrap() as f32;

        for (spaceship, transform, motion_2d) in
            (&mut spaceships, &mut transforms, &mut motion_2d_components).join()
        {
            //keep spaceship with bounds of arena
            spaceship.constrain_to_arena(transform, motion_2d);

            //if barrel rolling a direction use the barrel roll x velocity, otherwise accelerate normally
            if spaceship.barrel_action_left {
                motion_2d.velocity.x = -1.0 * spaceship.barrel_speed;
            } else if spaceship.barrel_action_right {
                motion_2d.velocity.x = spaceship.barrel_speed;
            } else {
                spaceship.accelerate(x_move, y_move, motion_2d);
            }

            spaceship.update_position(transform, time.delta_seconds(), motion_2d);
        }
    }
}
