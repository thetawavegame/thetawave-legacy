use crate::visual::FadeComponent;
use amethyst::{
    ecs::prelude::{Join, System, WriteStorage},
    renderer::resources::Tint,
};

/// Handles fading of entities with a FadeComponent
pub struct FadeSystem;

impl<'s> System<'s> for FadeSystem {
    type SystemData = (WriteStorage<'s, Tint>, WriteStorage<'s, FadeComponent>);

    fn run(&mut self, (mut tints, mut fades): Self::SystemData) {
        for (tint, fade) in (&mut tints, &mut fades).join() {
            if let Some(red_change) = &mut fade.red_change {
                if red_change.value <= red_change.max_value
                    && red_change.value >= red_change.min_value
                {
                    red_change.value += red_change.delta_value;
                    tint.0.red = red_change.value;
                }
            }

            if let Some(green_change) = &mut fade.green_change {
                if green_change.value <= green_change.max_value
                    && green_change.value >= green_change.min_value
                {
                    green_change.value += green_change.delta_value;
                    tint.0.green = green_change.value;
                }
            }

            if let Some(blue_change) = &mut fade.blue_change {
                if blue_change.value <= blue_change.max_value
                    && blue_change.value >= blue_change.min_value
                {
                    blue_change.value += blue_change.delta_value;
                    tint.0.blue = blue_change.value;
                }
            }

            if let Some(alpha_change) = &mut fade.alpha_change {
                if alpha_change.value <= alpha_change.max_value
                    && alpha_change.value >= alpha_change.min_value
                {
                    alpha_change.value += alpha_change.delta_value;
                    tint.0.alpha = alpha_change.value;
                }
            }
        }
    }
}
