use crate::{
    constants::{ARENA_MAX_Y, ITEM_SPAWN_Y_OFFSET},
    entities::SpawnableType,
    player::components::PlayerComponent,
    spawnable::resources::{ConsumablesResource, ItemsResource},
    visual::resources::SpriteSheetsResource,
};
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities, LazyUpdate, ReadExpect},
};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

pub type StockProbabilities = Vec<(SpawnableType, f32)>;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StoreResource {
    pub stock_probs: StockProbabilities,
    pub restock_timer: f32,
    pub restock_period: f32,
    pub inventory: Vec<Option<SpawnableType>>,
}

impl StoreResource {
    fn choose_stock(&mut self) {
        self.inventory = vec![None, None, None];
        let mut choose_pool = self.stock_probs.clone();

        // choose 3 items
        for i in 0..3 {
            let total_probs = choose_pool.iter().fold(0.0, |sum, item| sum + item.1);

            // choose an item
            let pos = thread_rng().gen::<f32>() * total_probs;
            let mut sum = 0.0;

            for (entity_type, value) in choose_pool.clone() {
                sum += value;
                if sum > pos {
                    //let item_to_add = &items_resource.item_entities[&item_type];
                    choose_pool.retain(|element| element != &(entity_type.clone(), value)); // remove chosen item from cloned choose pool
                    self.inventory[i] = Some(entity_type.clone());

                    let entity_index = self
                        .stock_probs
                        .iter()
                        .position(|element| element == &(entity_type.clone(), value))
                        .unwrap();

                    if let SpawnableType::Item(_) = entity_type {
                        self.stock_probs[entity_index].1 /= 2.0; // divide probability of item appearing in store by 2
                    }

                    break;
                }
            }
        }
    }

    pub fn purchase(
        &mut self,
        inventory_index: usize,
        entities: &Entities,
        player: &mut PlayerComponent,
        transform: &Transform,
        items_resource: &ReadExpect<ItemsResource>,
        consumables_resource: &ReadExpect<ConsumablesResource>,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) -> bool {
        if let Some(entity_type) = &self.inventory[inventory_index] {
            match entity_type {
                SpawnableType::Item(item_type) => {
                    let item_data = items_resource.item_entities[item_type].clone();
                    if player.money >= item_data.item_component.price {
                        player.money -= item_data.item_component.price;

                        let mut spawn_transform = Transform::default();
                        spawn_transform.set_translation_xyz(
                            transform.translation().x,
                            ARENA_MAX_Y + ITEM_SPAWN_Y_OFFSET,
                            0.0,
                        );

                        items_resource.spawn_item(
                            item_type,
                            false,
                            spawn_transform,
                            spritesheets_resource,
                            entities,
                            lazy_update,
                        );

                        for (i, e_type) in self.stock_probs.iter().enumerate() {
                            if e_type.0 == *entity_type {
                                self.stock_probs[i].1 = 0.0; //set probability of appearing again to 0
                                break;
                            }
                        }

                        self.inventory[inventory_index] = None; //change item slot data to None
                        return true;
                    }
                }
                SpawnableType::Consumable(consumable_type) => {
                    let consumable_data =
                        consumables_resource.consumable_entities[consumable_type].clone();
                    if player.money >= consumable_data.consumable_component.price {
                        player.money -= consumable_data.consumable_component.price;

                        let mut spawn_transform = Transform::default();
                        spawn_transform.set_translation_xyz(
                            transform.translation().x,
                            ARENA_MAX_Y + ITEM_SPAWN_Y_OFFSET,
                            0.0,
                        );

                        consumables_resource.spawn_consumable(
                            consumable_type,
                            false,
                            spawn_transform,
                            spritesheets_resource,
                            entities,
                            lazy_update,
                        );

                        self.inventory[inventory_index] = None; //change item slot data to None
                        return true;
                    }
                }
                _ => panic!("Only items and consumables can be purchased in the store."),
            }
        }
        false
    }

    pub fn restock_when_ready(&mut self, dt: f32) -> bool {
        if self.restock_timer > 0.0 {
            self.restock_timer -= dt;
        } else {
            self.restock_timer = self.restock_period;
            self.choose_stock();
            return true;
        }
        false
    }
}
