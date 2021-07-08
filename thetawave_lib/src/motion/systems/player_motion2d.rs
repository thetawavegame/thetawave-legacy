use crate::{motion::components::Motion2DComponent, player::PlayerComponent};
use amethyst::{
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

/// Handles motion of players
pub struct PlayerMotion2DSystem;

impl<'s> System<'s> for PlayerMotion2DSystem {
    type SystemData = (
        ReadStorage<'s, PlayerComponent>,
        WriteStorage<'s, Motion2DComponent>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (players, mut motion_2d_components, input): Self::SystemData) {
        let x_move = input.axis_value("player_x").unwrap() as f32;
        let y_move = input.axis_value("player_y").unwrap() as f32;

        for (_player, motion_2d) in (&players, &mut motion_2d_components).join() {
            handle_spaceship_movement(motion_2d, x_move, y_move);
        }
    }
}

/// Handles acceleration and deceleration of spaceship based on given x/y direction inputs
fn handle_spaceship_movement(motion: &mut Motion2DComponent, x_move: f32, y_move: f32) {
    // Handle deceleration in the x direction while moving.
    if x_move == 0.0 && motion.velocity.x != 0.0 {
        if motion.velocity.x > 0.0 {
            motion.velocity.x += -motion.deceleration.x;
        } else {
            motion.velocity.x += motion.deceleration.x;
        }
    }

    // Handle deceleration in the y direction while moving.
    if y_move == 0.0 && motion.velocity.y != 0.0 {
        if motion.velocity.y > 0.0 {
            motion.velocity.y += -motion.deceleration.y;
        } else {
            motion.velocity.y += motion.deceleration.y;
        }
    }

    // Accelerate in the x,y direction
    motion.velocity.x += x_move * motion.acceleration.x;
    motion.velocity.y += y_move * motion.acceleration.y;
}
