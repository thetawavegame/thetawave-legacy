use crate::{
    motion::components::Motion2DComponent,
    visual::components::{AnimationComponent, AnimationType, ColorChannelChange, FadeComponent},
};
use amethyst::{
    assets::Handle,
    core::{
        math::{Vector2, Vector3},
        transform::Transform,
    },
    ecs::{World, WorldExt},
    prelude::Builder,
    renderer::{palette::Srgba, resources::Tint, SpriteRender, SpriteSheet},
};

pub fn initialize_background(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(23500.0, 135.0, -15000.0);
    local_transform.set_scale(Vector3::new(65.0, 65.0, 0.0));

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    let motion_2d = Motion2DComponent {
        velocity: Vector2::new(-120.0, 0.0),
        acceleration: Vector2::new(0.0, 0.0),
        deceleration: Vector2::new(0.0, 0.0),
        angular_velocity: 0.0,
        angular_acceleration: 0.0,
        angular_deceleration: 0.0,
        angular_speed: 0.0,
        speed: Vector2::new(120.0, 0.0),
        max_speed: Vector2::new(120.0, 0.0),
        immovable: false,
        target_position: None,
    };

    let animation = AnimationComponent {
        start_idx: 0,
        frame_count: 6,
        current_frame: 0,
        frame_time: 0.2,
        elapsed_time: 0.0,
        forward: true,
        animation_type: AnimationType::PingPong,
    };

    let current_color_value = 0.05;

    let rgb_change = ColorChannelChange {
        delta_value: 0.00005,
        value: current_color_value,
        max_value: 0.58,
        min_value: 0.0,
    };

    let fade = FadeComponent {
        red_change: Some(rgb_change.clone()),
        green_change: Some(rgb_change.clone()),
        blue_change: Some(rgb_change),
        alpha_change: None,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(motion_2d)
        .with(local_transform)
        .with(animation)
        .with(Tint(Srgba::new(
            current_color_value,
            current_color_value,
            current_color_value,
            1.0,
        )))
        .with(fade)
        .build();
}
