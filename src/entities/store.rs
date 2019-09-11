use amethyst::{
    prelude::Builder,
    ecs::prelude::{World, Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{SpriteRender},
    core::{
        transform::Transform,
        math::Vector3,
    },
};
use crate::{
    resources::{SpriteResource, ItemPool},
    components::Store,
};

pub type StockProbabilities = Vec<(String, f32)>;

const RESTOCK_INTERVAL: f32 = 10.0;

pub fn initialise_store(world: &mut World) {

    let stock_list: StockProbabilities  = world.read_resource::<ItemPool>()
        .iter()
        .map(|(key,_)| (key.clone(), 1.0)) // all with same probability
        .collect();

    let mut store = Store {
        items: stock_list,
        restock_timer: RESTOCK_INTERVAL,
        restock_interval: RESTOCK_INTERVAL,
        item_inventory: [None, None, None],
        item_icons: [None, None, None],
        consumable_inventory: vec![],
    };

    world
        .create_entity()
        .with(store)
        .build();
}


