use amethyst::ecs::prelude::{Component, DenseVecStorage};
use crate::components::{Item, Consumable};
use crate::resources::{ItemPool};
use rand::{thread_rng, Rng};

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
    resources::{SpriteResource},
    space_shooter::{ARENA_MAX_X, ARENA_MIN_X, ARENA_MIN_Y, ARENA_MAX_Y},
};


// each item has a customizable chance of appearing in the store, to make certain items more rare
pub type StockProbabilities = Vec<(String, f32)>;

#[derive(Clone)]
pub struct Store {
    pub items: StockProbabilities,
    pub restock_timer: f32,
    pub restock_interval: f32,
    pub item_inventory: Vec<Item>,
    pub item_icons: Vec<Entity>,
    pub consumable_inventory: Vec<Consumable>,
}

impl Component for Store {
    type Storage = DenseVecStorage<Self>;
}

impl Store {

    pub fn choose_item_stock(&mut self, item_pool: ItemPool) {
        self.item_inventory = vec![];
        let mut choose_pool = self.items.clone();
        //let item_pool = world.read_resource::<ItemPool>();
        //add three items to item_inventory
        for _ in 0..3 {
            //println!("item pool: {:?}", choose_pool);
            let total_probs = choose_pool.iter().fold(0.0, |sum, item| sum + item.1);

            //choose an item
            let pos = thread_rng().gen::<f32>() * total_probs;
            let mut sum = 0.0;
            for (name, value) in choose_pool.clone() {
                sum += value;
                if sum > pos {
                    let item_to_add = &item_pool[&name];
                    ///println!("adding item to stock: {:?}", item_to_add);
                    choose_pool.retain(|element| element != &(name.clone(), value));
                    self.item_inventory.push(item_to_add.clone());

                    let item_index = self.items.iter().position(|element| element == &(name.clone(), value)).unwrap();
                    self.items[item_index].1 /= 2.0; //divide probability of appearing again by 2
                    break;
                }
            }
        }

    }

    pub fn restock(&mut self, dt: f32, item_pool: ItemPool, entities: &Entities, sprite_resource: &ReadExpect<SpriteResource>, lazy_update: &ReadExpect<LazyUpdate>) {
        if self.restock_timer > 0.0 {
            self.restock_timer -= dt;
        } else {
            self.restock_timer += self.restock_interval;
            self.choose_item_stock(item_pool);
            println!("store item stock: {:?}", self.item_inventory);
            println!("store item stock: {:?}", self.items);

            for item_icon in self.item_icons.iter() {
                entities.delete(*item_icon);
            }

            //add item images to gui
            for (i, item) in self.item_inventory.clone().iter().enumerate() {
                println!("item: {:?}", item);
                self.spawn_store_icon(entities, sprite_resource, lazy_update, i as f32, item.sprite_index);
            }

            
        }
    }

    pub fn spawn_store_icon(&mut self, entities: &Entities, sprite_resource: &ReadExpect<SpriteResource>, lazy_update: &ReadExpect<LazyUpdate>, index: f32, sprite_index: usize){
        let store_icon_entity: Entity = entities.create();

        let mut local_transform = Transform::default();
        local_transform.set_translation(Vector3::new(ARENA_MAX_X + 10.0 + 2.0, (ARENA_MIN_Y + 9.0 + 25.0 + 2.0) + (20.0 * index), 0.9));

        let sprite_render = SpriteRender {
            sprite_sheet: sprite_resource.sprite_sheet.clone(),
            sprite_number: sprite_index,
        };
        
        lazy_update.insert(store_icon_entity, sprite_render);
        lazy_update.insert(store_icon_entity, local_transform);

        self.item_icons.push(store_icon_entity);

    }
}
/*
// spawn random item with position, if timer has expired
    pub fn spawn_with_position(&mut self, dt: f32) -> Option<(f32, &String)> {
        if self.timer > 0.0 || self.counter == 0 {
            self.timer -= dt;
            None
        } else {
            self.timer += self.interval;
            self.counter -= 1;
            Some((
                choose_position(),
                choose_name_precalculated(self.prob_space, &self.probabilities),
            ))
        }
    }
    /// disable specified item from spawning, at the same time increases all other items spawn rate
    /// if all items are disabled, `spawn_with_position` will always return None
    pub fn disable_item(&mut self, item: &String) {
        match self.probabilities.iter_mut().find(|(name, _)| name == item) {
            Some((_, prob)) => {
                *prob = 0.0;
                self.prob_space = calculate_total_probabilities(&self.probabilities);
                if self.prob_space == 0.0 {
                    self.counter = 0;
                }
            }
            _ => {}
        }
    }

    pub fn choose_random_name(probs: &SpawnProbabilities) -> &String {
    choose_name_precalculated(calculate_total_probabilities(&probs), &probs)
}

fn choose_position() -> f32 {
    let max_width = ARENA_MAX_X - ARENA_SPAWN_OFFSET;
    let min_width = ARENA_MIN_X + ARENA_SPAWN_OFFSET;
    ARENA_MIN_X + ARENA_SPAWN_OFFSET + thread_rng().gen::<f32>() * (max_width - min_width)
}

*/



