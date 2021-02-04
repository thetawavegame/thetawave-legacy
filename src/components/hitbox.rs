use amethyst::{
    core::{math::Vector2, transform::Transform},
    ecs::prelude::{Component, DenseVecStorage},
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Hitbox2DComponent {
    pub width: f32,
    pub height: f32,
    #[serde(default = "des_offset")]
    pub offset: Vector2<f32>,
    #[serde(default = "des_offset_rotation")]
    pub offset_rotation: f32, // offset in radians
}
fn des_offset() -> Vector2<f32> {
    Vector2::new(0.0, 0.0)
}

fn des_offset_rotation() -> f32 {
    0.0
}

impl Component for Hitbox2DComponent {
    type Storage = DenseVecStorage<Self>;
}

impl Hitbox2DComponent {
    pub fn set_offset_rotation(&mut self, current_angle: f32) {
        self.offset_rotation = -current_angle;
    }

    pub fn is_colliding(
        &self,
        hitbox_b: &Hitbox2DComponent,
        transform_a: &Transform,
        transform_b: &Transform,
    ) -> bool {
        if self.offset_rotation == 0.0 && hitbox_b.offset_rotation == 0.0 {
            let x1 = transform_a.translation().x - (self.width / 2.0) + self.offset.x;
            let y1 = transform_a.translation().y - (self.height / 2.0) + self.offset.y;
            let x2 = transform_b.translation().x - (hitbox_b.width / 2.0) + hitbox_b.offset.x;
            let y2 = transform_b.translation().y - (hitbox_b.height / 2.0) + hitbox_b.offset.y;

            return x1 < (x2 + hitbox_b.width)
                && (x1 + self.width) > x2
                && y1 < (y2 + hitbox_b.height)
                && (y1 + self.height) > y2;
        }

        // Step 1: find coordinates of all four corners of each hitbox centered at origin
        let a_ur_x_temp = self.width / 2.0;
        let a_ur_y_temp = self.height / 2.0;
        let a_ul_x_temp = -self.width / 2.0;
        let a_ul_y_temp = self.height / 2.0;
        let a_lr_x_temp = self.width / 2.0;
        let a_lr_y_temp = -self.height / 2.0;
        let a_ll_x_temp = -self.width / 2.0;
        let a_ll_y_temp = -self.height / 2.0;

        let b_ur_x_temp = hitbox_b.width / 2.0;
        let b_ur_y_temp = hitbox_b.height / 2.0;
        let b_ul_x_temp = -hitbox_b.width / 2.0;
        let b_ul_y_temp = hitbox_b.height / 2.0;
        let b_lr_x_temp = hitbox_b.width / 2.0;
        let b_lr_y_temp = -hitbox_b.height / 2.0;
        let b_ll_x_temp = -hitbox_b.width / 2.0;
        let b_ll_y_temp = -hitbox_b.height / 2.0;

        // Step 2: find rotated coordinates of four corners
        let a_x_offset = transform_a.translation().x + self.offset.x;
        let a_y_offset = transform_a.translation().y + self.offset.y;
        let rotated_hitbox_1 = [
            Vector(
                rotate_x(a_ur_x_temp, a_ur_y_temp, self.offset_rotation) + a_x_offset,
                rotate_y(a_ur_x_temp, a_ur_y_temp, self.offset_rotation) + a_y_offset,
            ),
            Vector(
                rotate_x(a_ul_x_temp, a_ul_y_temp, self.offset_rotation) + a_x_offset,
                rotate_y(a_ul_x_temp, a_ul_y_temp, self.offset_rotation) + a_y_offset,
            ),
            Vector(
                rotate_x(a_lr_x_temp, a_lr_y_temp, self.offset_rotation) + a_x_offset,
                rotate_y(a_lr_x_temp, a_lr_y_temp, self.offset_rotation) + a_y_offset,
            ),
            Vector(
                rotate_x(a_ll_x_temp, a_ll_y_temp, self.offset_rotation) + a_x_offset,
                rotate_y(a_ll_x_temp, a_ll_y_temp, self.offset_rotation) + a_y_offset,
            ),
        ];

        let b_x_offset = transform_b.translation().x + hitbox_b.offset.x;
        let b_y_offset = transform_b.translation().y + hitbox_b.offset.y;
        let rotated_hitbox_2 = [
            Vector(
                rotate_x(b_ur_x_temp, b_ur_y_temp, hitbox_b.offset_rotation) + b_x_offset,
                rotate_y(b_ur_x_temp, b_ur_y_temp, hitbox_b.offset_rotation) + b_y_offset,
            ),
            Vector(
                rotate_x(b_ul_x_temp, b_ul_y_temp, hitbox_b.offset_rotation) + b_x_offset,
                rotate_y(b_ul_x_temp, b_ul_y_temp, hitbox_b.offset_rotation) + b_y_offset,
            ),
            Vector(
                rotate_x(b_lr_x_temp, b_lr_y_temp, hitbox_b.offset_rotation) + b_x_offset,
                rotate_y(b_lr_x_temp, b_lr_y_temp, hitbox_b.offset_rotation) + b_y_offset,
            ),
            Vector(
                rotate_x(b_ll_x_temp, b_ll_y_temp, hitbox_b.offset_rotation) + b_x_offset,
                rotate_y(b_ll_x_temp, b_ll_y_temp, hitbox_b.offset_rotation) + b_y_offset,
            ),
        ];

        // run separating axis theorem
        sat_is_colliding(&rotated_hitbox_1, &rotated_hitbox_2, &None)
    }
}

fn rotate_x(x: f32, y: f32, angle: f32) -> f32 {
    (x * angle.cos()) + (y * angle.sin())
}

fn rotate_y(x: f32, y: f32, angle: f32) -> f32 {
    (-x * angle.sin()) + (y * angle.cos())
}

/*
The code below is from this repository: https://github.com/JoelEager/Rust-Collision-Detector

MIT License

Copyright (c) 2018 Joel Eager

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

#[derive(Clone, Copy, Debug)]
pub struct Vector(pub f32, pub f32);

pub fn sat_is_colliding(poly1: &[Vector], poly2: &[Vector], max_dist: &Option<f32>) -> bool {
    let estimated_dist = (poly1[1].0 - poly2[0].0).powi(2) + (poly1[1].1 - poly2[0].1).powi(2);
    match max_dist {
        &Some(max_dist) if estimated_dist > max_dist.powi(2) => false,
        &Some(_) | &None => run_sat(poly1, poly2),
    }
}

fn run_sat(poly1: &[Vector], poly2: &[Vector]) -> bool {
    let mut edges = Vec::new();
    edges.append(&mut poly_to_edges(&poly1));
    edges.append(&mut poly_to_edges(&poly2));

    let axes = edges.into_iter().map(orthogonal);

    for axis in axes {
        if !overlap(project(&poly1, axis), project(&poly2, axis)) {
            return false;
        }
    }

    true
}

fn edge_vector(point1: Vector, point2: Vector) -> Vector {
    Vector(point2.0 - point1.0, point2.1 - point1.1)
}

fn poly_to_edges(poly: &[Vector]) -> Vec<Vector> {
    // Returns a Vec of the edges of the poly as vectors
    let mut edges = Vec::with_capacity(poly.len());

    for index in 0..poly.len() {
        edges.push(edge_vector(poly[index], poly[(index + 1) % poly.len()]));
    }

    edges
}

fn orthogonal(vector: Vector) -> Vector {
    Vector(vector.1, -vector.0)
}

fn dot_product(vector1: Vector, vector2: Vector) -> f32 {
    vector1.0 * vector2.0 + vector1.1 * vector2.1
}

fn project(poly: &[Vector], axis: Vector) -> Vector {
    let mut min: Option<f32> = None;
    let mut max: Option<f32> = None;

    for point in poly.iter() {
        let dot = dot_product(*point, axis);

        match min {
            Some(val) if val < dot => (),
            _ => min = Some(dot),
        }
        match max {
            Some(val) if val > dot => (),
            _ => max = Some(dot),
        }
    }

    Vector(min.unwrap(), max.unwrap())
}

fn overlap(projection1: Vector, projection2: Vector) -> bool {
    projection1.0 <= projection2.1 && projection2.0 <= projection1.1
}
