use crate::{
    components::{Hitbox2DComponent, Motion2DComponent, PlayerComponent},
    constants::{ARENA_MAX_X, ARENA_MAX_Y, ARENA_MIN_X, ARENA_MIN_Y},
};
use amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};
pub struct SpaceshipMovementSystem;

impl<'s> System<'s> for SpaceshipMovementSystem {
    type SystemData = (
        ReadStorage<'s, PlayerComponent>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Motion2DComponent>,
        ReadStorage<'s, Hitbox2DComponent>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(
        &mut self,
        (players, mut transforms, mut motion_2d_components, hitboxes, input): Self::SystemData,
    ) {
        let x_move = input.axis_value("player_x").unwrap() as f32;
        let y_move = input.axis_value("player_y").unwrap() as f32;

        for (_player, transform, motion_2d, hitbox) in (
            &players,
            &mut transforms,
            &mut motion_2d_components,
            &hitboxes,
        )
            .join()
        {
            handle_spaceship_movement(motion_2d, x_move, y_move);

            constrain_spaceship_to_arena(motion_2d, transform, hitbox)
        }
    }
}

// Handles acceleration and deceleration of spaceship based on given x,y direction inputs.
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

fn constrain_spaceship_to_arena(
    motion: &mut Motion2DComponent,
    transform: &Transform,
    hitbox: &Hitbox2DComponent,
) {
    //let x_pos = transform.translation().x - (hitbox.width / 2.0);
    let y_pos = transform.translation().y - (hitbox.height / 2.0);

    /*
    if x_pos < ARENA_MIN_X || x_pos > ARENA_MAX_X {
        motion.velocity.x *= -1.0;
    }
    */

    if y_pos < ARENA_MIN_Y || y_pos > ARENA_MAX_Y {
        motion.velocity.y *= -1.0;
    }
}
