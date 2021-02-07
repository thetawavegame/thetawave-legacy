use crate::{components::StoreComponent, constants::RESTOCK_INTERVAL, resources::ItemsResource};
use amethyst::{
    ecs::{World, WorldExt},
    prelude::Builder,
};

pub type StockProbabilities = Vec<(String, f32)>;

pub fn initialize_store(world: &mut World) {
    let stock_list: StockProbabilities = world
        .read_resource::<ItemsResource>()
        .item_entities
        .iter()
        .map(|(key, _)| (key.clone(), 1.0)) // all with same probability
        .collect();

    let store = StoreComponent {
        items: stock_list,
        restock_timer: RESTOCK_INTERVAL,
        restock_interval: RESTOCK_INTERVAL,
        item_inventory: [None, None, None],
        item_icons: [None, None, None],
        consumable_inventory: vec![],
    };

    world.create_entity().with(store).build();
}
