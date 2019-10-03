use amethyst::{
    gltf::{GltfSceneAsset, GltfSceneFormat},
    prelude::Builder,
    ecs::{World, WorldExt},
    core::transform::Transform,
    assets::{Handle, AssetStorage, Loader},
    core::math::Vector3,
};
use crate::{
    constants::{ARENA_MIN_X, ARENA_WIDTH, ARENA_HEIGHT},
    components::Planet,
};

pub fn initialise_planet(world: &mut World, gltf_path: &str, x: f32, y: f32, z: f32, scale:f32, angle: f32, d_angle: f32) {

    let planet = Planet {
        angle: angle,
        d_angle: d_angle,
    };

    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(x, y, z);
    local_transform.set_scale(Vector3::new(scale, scale, scale));
    local_transform.set_rotation_euler(0.0,planet.angle.to_radians(),75_f32.to_radians());

    let gltf = {
        let loader = world.read_resource::<Loader>();
        let gltf_storage = world.read_resource::<AssetStorage<GltfSceneAsset>>();
            loader.load(
                format!("mesh/{}", gltf_path),
                GltfSceneFormat::default(),
                (),
                &gltf_storage,
            )
    };

    world
        .create_entity()
        .with(planet)
        .with(gltf)
        .with(local_transform)
        .build();
}
