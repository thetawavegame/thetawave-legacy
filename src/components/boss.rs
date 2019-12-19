use amethyst::{
    ecs::prelude::{
        Component,
        DenseVecStorage,
        Entity,
    }
};

use crate::{
    components::{Enemy},
};

pub struct Repeater {
    pub body: Entity
}

impl Component for Repeater {
    type Storage = DenseVecStorage<Self>;
}
