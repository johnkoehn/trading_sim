use std::thread;
use std::time::Duration;
use rand::seq::IteratorRandom;
use std::collections::HashMap;
use std::slice::Iter;
use self::ResourceType::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ResourceType {
    Gold,
    Food,
    Wood,
    Stone,
    Villager
}

impl ResourceType {
    pub fn iterator() -> Iter<'static, ResourceType> {
        static RESOURCE_TYPES: [ResourceType; 5] = [Gold, Food, Wood, Stone, Villager];
        RESOURCE_TYPES.iter()
    }
}

#[derive(Debug)]
pub struct Village {
    pub resources: HashMap<ResourceType, u64>
}

impl Village {
    pub fn new() -> Village {
        let mut village = Village {
            resources: HashMap::new()
        };

        for resource_type in ResourceType::iterator() {
            village.resources.insert(*resource_type, 0);
        }

        village
    }

    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_secs(10));

        let resource_type = ResourceType::iterator().choose(&mut rng).unwrap();

        let new_value = self.resources.get(resource_type).unwrap() + 1;
        self.resources.insert(*resource_type, new_value);
    }
}