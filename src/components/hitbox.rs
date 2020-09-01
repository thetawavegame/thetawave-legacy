use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
};

pub struct Hitbox2DComponent {
    pub width: f32,
    pub height: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub offset_rotation: f32, // offset in radians
}

impl Component for Hitbox2DComponent {
    type Storage = DenseVecStorage<Self>;
}

impl Hitbox2DComponent {
    pub fn is_colliding(
        &self,
        transform_a: &Transform,
        hitbox_b: &Hitbox2DComponent,
        transform_b: &Transform,
    ) -> bool {
        if self.offset_rotation == 0.0 && hitbox_b.offset_rotation == 0.0 {
            let x1 = transform_a.translation().x - (self.width / 2.0) - self.offset_x;
            let y1 = transform_a.translation().y - (self.height / 2.0) - self.offset_y;
            let x2 = transform_b.translation().x - (hitbox_b.width / 2.0) - hitbox_b.offset_x;
            let y2 = transform_b.translation().y - (hitbox_b.height / 2.0) - hitbox_b.offset_y;

            return x1 < (x2 + hitbox_b.width)
                && (x1 + self.width) > x2
                && y1 < (y2 + hitbox_b.height)
                && (y1 + self.height) > y2;
        }

        false // add additional formula or generalize existing formula to account for hitboxes with a offset_rotation greater than 0
    }
}
