use crate::{
    components::{Hitbox2DComponent, Motion2DComponent},
    constants::{ARENA_MAX_X, ARENA_MAX_Y, ARENA_MIN_X, ARENA_MIN_Y},
};
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
};

pub struct ConstrainToArenaSystem;

impl<'s> System<'s> for ConstrainToArenaSystem {
    type SystemData = (
        ReadStorage<'s, Hitbox2DComponent>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Motion2DComponent>,
    );

    fn run(&mut self, (hitboxes, mut transforms, mut motions): Self::SystemData) {
        for (hitbox, transform, motion) in (&hitboxes, &mut transforms, &mut motions).join() {
            let x_pos = transform.translation().x - (hitbox.width / 2.0);
            let y_pos = transform.translation().y - (hitbox.height / 2.0);

            if x_pos < ARENA_MIN_X || x_pos > ARENA_MAX_X {
                motion.velocity.x *= -1.0;
                motion.acceleration.x *= -1.0;
            }

            if y_pos < ARENA_MIN_Y || y_pos > ARENA_MAX_Y {
                motion.velocity.y *= -1.0;
                motion.acceleration.y *= -1.0;
            }
        }
    }
}
