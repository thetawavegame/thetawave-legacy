use crate::constants::STATUS_BAR_Z;
use amethyst::{
    core::math::{Vector2, Vector3},
    ecs::prelude::{Component, DenseVecStorage, Entities, Entity},
};
use std::{cmp::Ordering, vec::Vec};

/// Type of status bar
pub enum StatusType {
    Health,
    Defense,
    Roll,
    Restock,
}

/// Used for managing status bars
pub struct StatusBarComponent {
    /// Type of status bar
    pub status_type: StatusType,
    /// Position of the status bar
    pub position: Vector2<f32>,
    /// Vector of status bar unit entities
    pub status_unit_stack: Vec<Entity>,
    /// Total unit capacity
    pub unit_limit: f32,
}

impl Component for StatusBarComponent {
    type Storage = DenseVecStorage<Self>;
}

impl StatusBarComponent {
    /// Update the status bar units in the x-direction
    pub fn update_units_x(
        &mut self,
        max_value: f32,
        current_value: f32,
        entities: &Entities<'_>,
    ) -> Option<Vector3<f32>> {
        let status_divisor = max_value / self.unit_limit;
        let status_unit_num = (current_value / status_divisor).ceil() as usize;

        match status_unit_num.cmp(&self.status_unit_stack.len()) {
            Ordering::Greater => {
                let status_position = Vector3::new(self.position.x, self.position.y, STATUS_BAR_Z);
                self.position.x += 1.0;
                Some(status_position)
            }
            Ordering::Less => {
                if let Some(unit) = self.status_unit_stack.pop() {
                    entities.delete(unit).expect("unable to delete entity");
                    self.position.x -= 1.0;
                }
                None
            }
            Ordering::Equal => None,
        }
    }

    /// Update the status bar units in the y-direction
    pub fn update_units_y(
        &mut self,
        max_value: f32,
        current_value: f32,
        entities: &Entities<'_>,
    ) -> Option<Vector3<f32>> {
        let status_divisor = max_value / self.unit_limit;
        let status_unit_num = (current_value / status_divisor).ceil() as usize;

        match status_unit_num.cmp(&self.status_unit_stack.len()) {
            Ordering::Greater => {
                let status_position = Vector3::new(self.position.x, self.position.y, STATUS_BAR_Z);
                self.position.y += 1.0;
                Some(status_position)
            }
            Ordering::Less => {
                if let Some(unit) = self.status_unit_stack.pop() {
                    entities.delete(unit).expect("unable to delete entity");
                    self.position.y -= 1.0;
                }
                None
            }
            Ordering::Equal => None,
        }
    }
}
