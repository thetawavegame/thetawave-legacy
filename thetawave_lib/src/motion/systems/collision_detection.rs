use crate::{
    events::{ArenaBorderCollisionEvent, CollisionEvent, MobCollisionEvent, PlayerCollisionEvent},
    misc::components::BarrierComponent,
    misc::resources::DebugLinesConfig,
    motion::components::{Hitbox2DComponent, Motion2DComponent},
    player::PlayerComponent,
    spawnable::MobComponent,
};
use amethyst::{
    core::{
        math::{UnitQuaternion, Vector2},
        transform::Transform,
    },
    ecs::*,
    renderer::debug_drawing::DebugLines,
    shrev::{EventChannel, ReaderId},
};

/// Handles detection of collisions between entities
pub struct CollisionDetectionSystem;

impl<'s> System<'s> for CollisionDetectionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Hitbox2DComponent>,
        ReadStorage<'s, Transform>,
        Write<'s, EventChannel<CollisionEvent>>,
        Write<'s, DebugLines>,
        Read<'s, DebugLinesConfig>,
    );

    fn run(
        &mut self,
        (
            entities,
            hitbox2ds,
            transforms,
            mut collision_channel,
            mut debug_lines,
            debug_lines_config,
        ): Self::SystemData,
    ) {
        for (entity_a, transform_a, hitbox_a) in (&entities, &transforms, &hitbox2ds).join() {
            for (entity_b, transform_b, hitbox_b) in (&entities, &transforms, &hitbox2ds).join() {
                if entity_a == entity_b {
                    continue;
                }

                if hitbox_a.is_colliding(hitbox_b, transform_a, transform_b) {
                    collision_channel.single_write(CollisionEvent::new(entity_a, entity_b));
                }
            }
            if cfg!(debug_assertions) {
                // draw debug lines for hitboxes
                debug_lines.draw_rotated_box(
                    [
                        transform_a.translation().x + hitbox_a.offset.x - (hitbox_a.width / 2.0),
                        transform_a.translation().y + hitbox_a.offset.y - (hitbox_a.height / 2.0),
                        transform_a.translation().z,
                    ]
                    .into(),
                    [
                        transform_a.translation().x + hitbox_a.offset.x + (hitbox_a.width / 2.0),
                        transform_a.translation().y + hitbox_a.offset.y + (hitbox_a.height / 2.0),
                        transform_a.translation().z,
                    ]
                    .into(),
                    UnitQuaternion::from_euler_angles(0.0, 0.0, -hitbox_a.offset_rotation),
                    debug_lines_config.hitbox_color,
                );
            }
        }
    }
}

/// Handles routing of collision data to specific event channels
#[derive(Default)]
pub struct CollisionHandlerSystem {
    event_reader: Option<ReaderId<CollisionEvent>>,
}

impl<'s> System<'s> for CollisionHandlerSystem {
    type SystemData = (
        ReadStorage<'s, PlayerComponent>,
        ReadStorage<'s, MobComponent>,
        ReadStorage<'s, BarrierComponent>,
        ReadStorage<'s, Motion2DComponent>,
        Read<'s, EventChannel<CollisionEvent>>,
        Write<'s, EventChannel<PlayerCollisionEvent>>,
        Write<'s, EventChannel<MobCollisionEvent>>,
        Write<'s, EventChannel<ArenaBorderCollisionEvent>>,
    );

    fn run(
        &mut self,
        (
            players,
            mobs,
            barriers,
            motions,
            collision_channel,
            mut player_collision_channel,
            mut mob_collision_channel,
            mut arena_border_collision_channel,
        ): Self::SystemData,
    ) {
        for event in collision_channel.read(self.event_reader.as_mut().unwrap()) {
            let mut collision_velocity: Option<Vector2<f32>> = None;
            let mut collider_immovable = false;

            if let Some(motion_component) = motions.get(event.entity_b) {
                collision_velocity = Some(motion_component.velocity);
                collider_immovable = motion_component.immovable;
            }

            if let Some(_player) = players.get(event.entity_a) {
                player_collision_channel.single_write(PlayerCollisionEvent::new(
                    event.entity_a,
                    event.entity_b,
                    collider_immovable,
                    collision_velocity,
                ));
            } else if let Some(_mob) = mobs.get(event.entity_a) {
                mob_collision_channel.single_write(MobCollisionEvent::new(
                    event.entity_a,
                    event.entity_b,
                    collider_immovable,
                    collision_velocity,
                ));
            } else if let Some(_arena_border) = barriers.get(event.entity_a) {
                arena_border_collision_channel.single_write(ArenaBorderCollisionEvent::new(
                    event.entity_a,
                    event.entity_b,
                    collision_velocity,
                ));
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.event_reader = Some(
            world
                .fetch_mut::<EventChannel<CollisionEvent>>()
                .register_reader(),
        );
    }
}
