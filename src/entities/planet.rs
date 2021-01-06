use crate::components::PlanetComponent;
use amethyst::{
    assets::{AssetStorage, Loader, ProgressCounter},
    core::math::Vector3,
    core::transform::Transform,
    ecs::{World, WorldExt},
    gltf::{GltfSceneAsset, GltfSceneFormat},
    prelude::Builder,
};

pub fn initialize_planet(
    progress_counter: &mut ProgressCounter,
    world: &mut World,
    gltf_path: &str,
    x: f32,
    y: f32,
    z: f32,
    scale: f32,
    angle: f32,
    d_angle: f32,
) {
    let planet = PlanetComponent { angle, d_angle };

    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(x, y, z);
    local_transform.set_scale(Vector3::new(scale, scale, scale));
    local_transform.set_rotation_euler(0.0, planet.angle.to_radians(), 75_f32.to_radians());

    let gltf = {
        let loader = world.read_resource::<Loader>();
        let gltf_storage = world.read_resource::<AssetStorage<GltfSceneAsset>>();
        loader.load(
            format!("mesh/{}", gltf_path),
            GltfSceneFormat::default(),
            &mut *progress_counter,
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
