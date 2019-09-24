use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::core::Transform;

#[derive(Clone)]
pub struct Planet {
    pub angle: f32,
    pub d_angle: f32,
}

impl Component for Planet {
    type Storage = DenseVecStorage<Self>;
}

impl Planet {
    pub fn rotate(&mut self, transform: &mut Transform) {
        transform.set_rotation_euler(0.0,self.angle.to_radians() + self.d_angle, 75_f32.to_radians());
        self.angle -= self.d_angle;
        if self.angle < 0.0 {
            self.angle += 360.0;
        }
    }
}