use crate::components::{DefenseTag, HealthComponent};
use amethyst::ecs::prelude::{Join, ReadStorage, System, WriteStorage};

pub struct DefenseSystem;

impl<'s> System<'s> for DefenseSystem {
    type SystemData = (
        ReadStorage<'s, DefenseTag>,
        WriteStorage<'s, HealthComponent>,
    );

    fn run(&mut self, (defense_tags, mut healths): Self::SystemData) {
        for (defense_tag, health) in (&defense_tags, &mut healths).join() {
            health.constrain();
        }
    }
}
