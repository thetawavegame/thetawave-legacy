use amethyst::core::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

/// Component for managing planets motion
#[derive(Clone)]
pub struct PlanetComponent {
    /// Current angle of planet
    pub angle: f32,
    /// Change in angle of planet per frame
    pub d_angle: f32,
}

impl Component for PlanetComponent {
    type Storage = DenseVecStorage<Self>;
}

impl PlanetComponent {
    /// Rotate the planet
    pub fn rotate(&mut self, transform: &mut Transform) {
        transform.set_rotation_euler(
            0.0,
            self.angle.to_radians() + self.d_angle,
            75_f32.to_radians(),
        );
        self.angle -= self.d_angle;
        if self.angle < 0.0 {
            self.angle += 360.0;
        }
    }
}
