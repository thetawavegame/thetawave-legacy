use crate::{
    components::{Consumable, Item, Spaceship},
    constants::{ARENA_MAX_X, ARENA_MAX_Y, ARENA_MIN_Y, ITEM_SPAWN_Y_OFFSET},
    entities::spawn_item,
    resources::{ItemPool, SpriteResource},
};
use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{Component, DenseVecStorage, Entities, Entity, LazyUpdate, ReadExpect},
    renderer::SpriteRender,
};
use rand::{thread_rng, Rng};

// each item has a customizable chance of appearing in the store, to make certain items more rare
pub type StockProbabilities = Vec<(String, f32)>;

#[derive(Clone)]
pub struct Store {
    pub items: StockProbabilities,
    pub restock_timer: f32,
    pub restock_interval: f32,
    pub item_inventory: [Option<Item>; 3],
    pub item_icons: [Option<Entity>; 3],
    pub consumable_inventory: Vec<Consumable>,
}

impl Component for Store {
    type Storage = DenseVecStorage<Self>;
}

impl Store {
    pub fn choose_item_stock(&mut self, item_pool: ItemPool) {
        self.item_inventory = [None, None, None];
        let mut choose_pool = self.items.clone();

        //add three items to item_inventory
        for i in 0..3 {
            let total_probs = choose_pool.iter().fold(0.0, |sum, item| sum + item.1);

            //choose an item
            let pos = thread_rng().gen::<f32>() * total_probs;
            let mut sum = 0.0;
            for (name, value) in choose_pool.clone() {
                sum += value;
                if sum > pos {
                    let item_to_add = &item_pool[&name];
                    choose_pool.retain(|element| element != &(name.clone(), value));
                    self.item_inventory[i] = Some(item_to_add.clone());

                    let item_index = self
                        .items
                        .iter()
                        .position(|element| element == &(name.clone(), value))
                        .unwrap();
                    self.items[item_index].1 /= 2.0; //divide probability of appearing again by 2
                    break;
                }
            }
        }
    }

    pub fn restock(
        &mut self,
        dt: f32,
        item_pool: ItemPool,
        entities: &Entities,
        sprite_resource: &ReadExpect<SpriteResource>,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        if self.restock_timer > 0.0 {
            self.restock_timer -= dt;
        } else {
            self.restock_timer = self.restock_interval;
            self.choose_item_stock(item_pool);

            for item_icon in self.item_icons.iter() {
                if let Some(item_icon_to_delete) = item_icon {
                    let _res = entities.delete(*item_icon_to_delete);
                }
            }

            //add item images to gui
            for (i, item) in self.item_inventory.clone().iter().enumerate() {
                if let Some(store_item) = item {
                    //println!("store_item: {:?}", store_item);
                    self.spawn_store_icon(
                        entities,
                        sprite_resource,
                        lazy_update,
                        i as f32,
                        store_item.sprite_index,
                    );
                }
            }
        }
    }

    pub fn purchase_item(
        &mut self,
        item_index: usize,
        entities: &Entities,
        spaceship: &mut Spaceship,
        sprite_resource: &ReadExpect<SpriteResource>,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) -> bool {
        if let Some(item) = &self.item_inventory[item_index] {
            if spaceship.money >= item.price {
                println!(
                    "purchasing {} located in slot #{} for {}",
                    item.name, item_index, item.price
                );
                spaceship.money -= item.price;
                spawn_item(
                    entities,
                    sprite_resource,
                    item.clone(),
                    Vector3::new(spaceship.pos_x, ARENA_MAX_Y + ITEM_SPAWN_Y_OFFSET, 0.0),
                    lazy_update,
                );
                for (i, itm) in self.items.iter().enumerate() {
                    if itm.0 == item.name {
                        self.items[i].1 = 0.0; //set probability of appearing again to 0
                        break;
                    }
                }
                self.item_inventory[item_index] = None; //change item slot data to None
                if let Some(item_icon_to_destroy) = self.item_icons[item_index] {
                    let _res = entities.delete(item_icon_to_destroy); //remove store icon
                    self.item_icons[item_index] = None;
                }
                return true;
            }
        }
        false
    }

    pub fn spawn_store_icon(
        &mut self,
        entities: &Entities,
        sprite_resource: &ReadExpect<SpriteResource>,
        lazy_update: &ReadExpect<LazyUpdate>,
        index: f32,
        sprite_index: usize,
    ) {
        let store_icon_entity: Entity = entities.create();

        let mut local_transform = Transform::default();
        local_transform.set_translation(Vector3::new(
            ARENA_MAX_X + 10.0 + 2.0,
            (ARENA_MIN_Y + 32.0) + (40.0 - (19.0 * index)),
            0.9,
        ));

        let sprite_render = SpriteRender {
            sprite_sheet: sprite_resource.items_sprite_sheet.clone(),
            sprite_number: sprite_index,
        };

        lazy_update.insert(store_icon_entity, sprite_render);
        lazy_update.insert(store_icon_entity, local_transform);

        self.item_icons[index as usize] = Some(store_icon_entity);
    }
}
