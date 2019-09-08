use amethyst::ecs::prelude::{Component, DenseVecStorage, World};
use crate::components::{Item, Consumable};
use crate::resources::{ItemPool};
use rand::{thread_rng, Rng};


// each item has a customizable chance of appearing in the store, to make certain items more rare
pub type StockProbabilities = Vec<(String, f32)>;

#[derive(Clone)]
pub struct Store {
    pub items: StockProbabilities,
    pub item_inventory: Vec<Item>,
    pub consumable_inventory: Vec<Consumable>,
}

impl Component for Store {
    type Storage = DenseVecStorage<Self>;
}

impl Store {
    pub fn choose_item_stock(&mut self, world: &mut World) {
        //choose 3 random items, /= probability by 2
        let mut choose_pool = self.items.clone(); //copy pool
        let item_pool = world.read_resource::<ItemPool>();
        for _ in 0..3 {
            println!("item pool: {:?}", choose_pool);
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
                    //divide probability by 2
                    let item_index = self.items.iter().position(|element| element == &(name.clone(), value)).unwrap();
                    println!("added item index: {}", item_index);
                    self.items[item_index].1 /= 2.0;
                    break;
                }
            }
        }

    }


    fn calclate_total_probabilities(items: &StockProbabilities) -> f32 {
        items.iter().fold(0.0, |sum, item| sum + item.1)
    }

    fn choose_name_precalculated(total_probs: f32, items: &StockProbabilities) -> &String {
        // pos is in [0..total_probs)
        let pos = thread_rng().gen::<f32>() * total_probs;
        let mut sum = 0.0;
        for (name, value) in items {
            sum += value;
            if sum > pos {
                return &name;
            }
        }
        items
            .last()
            .map(|(name, _)| name)
            .expect("invalid probabilities, cannot choose name")
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



