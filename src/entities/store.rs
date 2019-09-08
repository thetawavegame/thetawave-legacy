use amethyst::{
    prelude::Builder,
    ecs::prelude::World,
};

use crate::{
    components::{Store},
    resources::ItemPool,
};

pub type StockProbabilities = Vec<(String, f32)>;

pub fn initialise_store(world: &mut World) {

    //add random items to stock
    // create spawn list, one item for each item type
    // add pop 3 items from the pool to the stock and remove from the pool
    let stock_list: StockProbabilities  = world.read_resource::<ItemPool>()
        .iter()
        .map(|(key,_)| (key.clone(), 1.0)) // all with same probability
        .collect();
    //println!("item pool: {:?}", stock_list);

    let mut store = Store {
        items: stock_list,
        item_inventory: vec![],
        consumable_inventory: vec![],
    };

    store.choose_item_stock(world);

    println!("store item stock: {:?}", store.item_inventory);
    println!("store item stock: {:?}", store.items);

    //add random consumables to stock
/*
    world
        .create_entity()
        .with(Defense{
            defense: DEFENSE,
            max_defense: DEFENSE,
        })
        .build();
        */
}