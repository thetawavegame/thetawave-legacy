use crate::{
    constants::{ARENA_MAX_Y, ITEM_SPAWN_Y_OFFSET},
    player::PlayerComponent,
    spawnable::{ConsumablesResource, ItemsResource, SpawnableType},
    tools::{weighted_rng, Timer},
    visual::SpriteSheetsResource,
};
use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{Entities, Entity, LazyUpdate, ReadExpect},
    prelude::Builder,
    renderer::SpriteRender,
};
use serde::{Deserialize, Serialize};
use std::convert::From;

/// Spawnables paired with probabilities of appearing
pub type StockProbabilities = Vec<(SpawnableType, f32)>;

/// Store configuration parameters stored in config file
#[derive(Serialize, Deserialize)]
pub struct StoreConfig {
    /// Period of time between refreshing the store
    restock_period: f32,
    /// Items and consumables stocked in the store
    stock: StockProbabilities,
}

/// Resource for managing inventory and allowing players to purchase items and consumables
pub struct StoreResource {
    /// Items and consumables available for purchase
    pub inventory: Vec<Option<SpawnableType>>,
    /// Timer for managing restocking the store
    pub timer: Timer,
    /// Items and consumables stocked in the store
    stock: StockProbabilities,
    /// Location coordinates of inventory icons
    // TODO: store positions of icons in config file
    icon_positions: Vec<Vector3<f32>>,
    /// Sprite entities representing the inventory items
    icon_entities: Vec<Option<Entity>>,
}

impl From<StoreConfig> for StoreResource {
    /// Converts StoreConfig to a StoreResource
    fn from(config: StoreConfig) -> Self {
        StoreResource::new(config.restock_period, config.stock)
    }
}

impl StoreResource {
    /// Create a new StoreResource from a restock period and stock of consumables and items
    pub fn new(restock_period: f32, stock: StockProbabilities) -> Self {
        StoreResource {
            inventory: vec![None, None, None],
            timer: Timer::new(restock_period),
            stock,
            icon_positions: vec![
                Vector3::new(327.0, 72.0, 0.9),
                Vector3::new(327.0, 53.0, 0.9),
                Vector3::new(327.0, 34.0, 0.9),
            ],
            icon_entities: vec![None, None, None],
        }
    }

    /// Remove icon entities
    fn destroy_icon(&mut self, index: usize, entities: &Entities) {
        if let Some(icon_entity) = self.icon_entities[index] {
            entities
                .delete(icon_entity)
                .expect("Unable to delete store icon entity.");
            self.icon_entities[index] = None;
        }
    }

    /// Create icon entity for item/consumable in inventory slot
    fn create_icon(
        &mut self,
        index: usize,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        items_resource: &ReadExpect<ItemsResource>,
        consumables_resource: &ReadExpect<ConsumablesResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        if let Some(spawnable_type) = self.inventory[index].clone() {
            // remove the existing icon
            self.destroy_icon(index, entities);

            // create the SpriteRender component
            let sprite_render = SpriteRender {
                sprite_sheet: spritesheets_resource.spritesheets
                    [&spawnable_type.get_spritesheet_name()]
                    .clone(),
                sprite_number: match spawnable_type {
                    SpawnableType::Consumable(consumable_type) => {
                        consumables_resource.consumable_entities[&consumable_type]
                            .sprite_render_data
                            .initial_index
                    }
                    SpawnableType::Item(item_type) => {
                        items_resource.item_entities[&item_type]
                            .sprite_render_data
                            .initial_index
                    }
                    _ => {
                        panic!("Attempted to set icon to invalid Spawnable. Icons must be set to items or consumables.")
                    }
                },
            };

            // create the Transform component
            let mut transform = Transform::default();
            transform.set_translation(self.icon_positions[index].into());

            // create the store icon entity
            let icon_entity = lazy_update
                .create_entity(entities)
                .with(sprite_render)
                .with(transform)
                .build();

            // store the entity in the icon_entities vector
            self.icon_entities[index] = Some(icon_entity);
        }
    }

    /// Update the store's timer and refresh stock
    pub fn update(
        &mut self,
        delta_time: f32,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        items_resource: &ReadExpect<ItemsResource>,
        consumables_resource: &ReadExpect<ConsumablesResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        if self.timer.update(delta_time) {
            // choose stock if timer resets
            self.choose_stock(
                spritesheets_resource,
                items_resource,
                consumables_resource,
                entities,
                lazy_update,
            );
        }
    }

    /// Choose new inventory using StockProbabilities
    fn choose_stock(
        &mut self,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        items_resource: &ReadExpect<ItemsResource>,
        consumables_resource: &ReadExpect<ConsumablesResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        self.inventory = vec![None, None, None];
        let mut choose_pool = self.stock.clone();

        // choose item for each slot
        for i in 0..3 {
            // isolate vector of probabilities
            let probs = choose_pool
                .iter()
                .map(|choose_pool| choose_pool.1)
                .collect();

            // choose an index with weighted rng
            let chosen_index = weighted_rng(probs);

            // get spawnable type at index
            let chosen_stock_type = choose_pool[chosen_index].0.clone();

            // remove spawnable from temporary pool
            choose_pool.remove(chosen_index);

            // set inventory slot to chosen type
            self.inventory[i] = Some(chosen_stock_type.clone());

            // create icon for chosen stock
            self.create_icon(
                i,
                spritesheets_resource,
                items_resource,
                consumables_resource,
                entities,
                lazy_update,
            );

            // divide probability of item appearing in store by 2
            if let SpawnableType::Item(_) = chosen_stock_type {
                self.stock[chosen_index].1 /= 2.0;
            }
        }
    }

    /// Execute purchase from store by player
    pub fn purchase(
        &mut self,
        inventory_index: usize,
        entities: &Entities,
        player: &mut PlayerComponent,
        transform: &Transform,
        items_resource: &ItemsResource,
        consumables_resource: &ConsumablesResource,
        spritesheets_resource: &SpriteSheetsResource,
        lazy_update: &LazyUpdate,
    ) -> bool {
        if let Some(entity_type) = self.inventory[inventory_index].clone() {
            // match item or consumable
            match &entity_type {
                SpawnableType::Item(item_type) => {
                    let item_data = items_resource.item_entities[item_type].clone();

                    // check if player has enough money
                    if player.money >= item_data.item_component.price {
                        player.money -= item_data.item_component.price;

                        // spawn item at top of arena above player
                        let mut spawn_transform = Transform::default();
                        spawn_transform.set_translation_xyz(
                            transform.translation().x,
                            ARENA_MAX_Y + ITEM_SPAWN_Y_OFFSET,
                            0.0,
                        );

                        items_resource.spawn_item(
                            item_type,
                            false,
                            &spawn_transform,
                            spritesheets_resource,
                            entities,
                            lazy_update,
                        );

                        // set probability of item being chosen again to 0
                        // TODO: address case where multiple items with different probabilities are in the pool
                        for (i, stock_probs) in self.stock.iter().enumerate() {
                            if stock_probs.0 == entity_type {
                                self.stock[i].1 = 0.0;
                                break;
                            }
                        }

                        // set store inventory slot to None
                        self.inventory[inventory_index] = None;

                        // destroy store icon
                        self.destroy_icon(inventory_index, entities);

                        return true;
                    }
                }
                SpawnableType::Consumable(consumable_type) => {
                    let consumable_data =
                        consumables_resource.consumable_entities[consumable_type].clone();

                    // check if player has enough money
                    if player.money >= consumable_data.consumable_component.price {
                        player.money -= consumable_data.consumable_component.price;

                        // spawn item at top of arena above player
                        let mut spawn_transform = Transform::default();
                        spawn_transform.set_translation_xyz(
                            transform.translation().x,
                            ARENA_MAX_Y + ITEM_SPAWN_Y_OFFSET,
                            0.0,
                        );

                        consumables_resource.spawn_consumable(
                            consumable_type,
                            false,
                            &spawn_transform,
                            spritesheets_resource,
                            entities,
                            lazy_update,
                        );

                        // set store inventory slot to None
                        self.inventory[inventory_index] = None;

                        // destroy existing store icon
                        self.destroy_icon(inventory_index, entities);

                        return true;
                    }
                }
                _ => panic!("Only items and consumables can be purchased in the store."),
            }
        }
        false
    }
}
