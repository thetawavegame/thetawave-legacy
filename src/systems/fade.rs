use crate::components::OpaqueFadeComponent;
use amethyst::{
    core::Transform,
    ecs::prelude::{Join, ReadStorage, System, WriteStorage},
    renderer::{palette::Srgba, resources::Tint},
};

pub struct OpaqueFadeSystem;

impl<'s> System<'s> for OpaqueFadeSystem {
    type SystemData = (
        WriteStorage<'s, Tint>,
        WriteStorage<'s, OpaqueFadeComponent>,
    );

    fn run(&mut self, (mut tints, mut opaque_fades): Self::SystemData) {
        for (tint, opaque_fade) in (&mut tints, &mut opaque_fades).join() {
            if opaque_fade.current_color_value < opaque_fade.max_color_value
                && opaque_fade.current_color_value > opaque_fade.min_color_value
            {
                opaque_fade.current_color_value += opaque_fade.color_change;
                tint.0.red = opaque_fade.current_color_value;
                tint.0.green = opaque_fade.current_color_value;
                tint.0.blue = opaque_fade.current_color_value;
                /*
                let new_tint = &mut Tint(Srgba::new(
                    opaque_fade.current_color_value,
                    opaque_fade.current_color_value,
                    opaque_fade.current_color_value,
                    1.0,
                ));
                */

                //tint = new_tint;
            }
        }
    }
}
