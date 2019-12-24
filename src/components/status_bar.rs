use amethyst::{
    ecs::prelude::{Component, DenseVecStorage, Entity, Entities},
    core::math::Vector3,
};
use std::{cmp::Ordering, vec::Vec};
use crate::{
    constants::STATUS_BAR_Z,
};

pub enum StatusType {
    Health,
    Defense,
    Roll,
    Restock,
}

pub struct StatusBar {
    pub status_type: StatusType,
    pub x_pos: f32,
    pub y_pos: f32,
    pub status_unit_stack: Vec<Entity>,
    pub unit_limit: f32,
}

impl Component for StatusBar {
    type Storage = DenseVecStorage<Self>;
}

impl StatusBar {

    pub fn update_units_x(&mut self, max_value: f32, current_value: f32, entities: &Entities) -> Option<Vector3<f32>> {

        let status_divisor = max_value / self.unit_limit;
        let status_unit_num = (current_value / status_divisor).ceil() as usize;

        match status_unit_num.cmp(&self.status_unit_stack.len()) {
            Ordering::Greater => {
                let status_position = Vector3::new(self.x_pos, self.y_pos, STATUS_BAR_Z);
                self.x_pos += 1.0;
                return Some(status_position);
            }
            Ordering::Less => {
                if let Some(unit) = self.status_unit_stack.pop() {
                    let _result = entities.delete(unit);
                    self.x_pos -= 1.0;
                }
                return None;
            }
            Ordering::Equal => return None,
        }
    }

    pub fn update_units_y(&mut self, max_value: f32, current_value: f32, entities: &Entities) -> Option<Vector3<f32>> {

        let status_divisor = max_value / self.unit_limit;
        let status_unit_num = (current_value / status_divisor).ceil() as usize;

        match status_unit_num.cmp(&self.status_unit_stack.len()) {
            Ordering::Greater => {
                let status_position = Vector3::new(self.x_pos, self.y_pos, STATUS_BAR_Z);
                self.y_pos += 1.0;
                return Some(status_position);
            }
            Ordering::Less => {
                if let Some(unit) = self.status_unit_stack.pop() {
                    let _result = entities.delete(unit);
                    self.y_pos -= 1.0;
                }
                return None;
            }
            Ordering::Equal => return None,
        }
    }
}
