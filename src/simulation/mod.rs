use std::vec::Vec;
use std::fs;
use serde_json;
use serde_yaml;
use std::error::Error;
use crate::price_data::{PriceData, PriceDataRaw};
use crate::config::Config;

#[derive(Debug)]
pub struct Simulation {
    price_history: Vec<PriceData>,
    // config: Config
}

// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
impl Simulation {
    pub fn new(path_to_price_history: &str, path_to_config: &str) -> Result<Simulation, Box<dyn Error>> {
        let price_history_as_json = fs::read_to_string(path_to_price_history)?;
        let config_as_yaml = fs::read_to_string(path_to_config)?;

        let mut simulation = Simulation {
            price_history: Vec::<PriceData>::new(),
        };

        let price_data: Vec<PriceDataRaw> = serde_json::from_str(&price_history_as_json.as_str())?;
        simulation.price_history = price_data.iter().map(|x| PriceData::new(x)).collect();

        let config_data: Config = serde_yaml::from_str(&config_as_yaml.as_str())?;

        Ok(simulation)
    }

    pub fn state(self) {
        println!("Running")
    }
}