use std::vec::Vec;
use std::fs;
use serde_json;
use crate::price_data::{PriceData, PriceDataRaw};

#[derive(Debug)]
pub struct Simulation {
    price_history: Vec<PriceData>
}

impl Simulation {
    pub fn new(path_to_price_history: &str) -> Simulation {
        let price_history_as_json = fs::read_to_string(path_to_price_history).unwrap();

        let mut simulation = Simulation {
            price_history: Vec::<PriceData>::new()
        };

        let result = serde_json::from_str(&price_history_as_json.as_str());

        if result.is_ok() {
            let price_data: Vec<PriceDataRaw> = result.unwrap();
            simulation.price_history = price_data.iter().map(|x| PriceData::new(x)).collect();
        } else {
            panic!("Invalid Price Data when loading simulation!")
        }

        simulation
    }

    pub fn state(self) {
        println!("Running")
    }
}