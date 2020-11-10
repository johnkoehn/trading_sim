use std::vec::Vec;
use std::fs;
use serde_json;
use serde_yaml;
use std::error::Error;
use crate::bot::Bot;
use crate::price_data::{PriceData, PriceDataRaw};
use crate::config::Config;

#[derive(Debug)]
pub struct Simulation {
    price_history: Vec<PriceData>,
    config: Config,
    bots: Vec<Bot>
}

// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
impl Simulation {
    pub fn new(path_to_price_history: &str, path_to_config: &str) -> Result<Simulation, Box<dyn Error>> {
        let price_history_as_json = fs::read_to_string(path_to_price_history)?;
        let config_as_yaml = fs::read_to_string(path_to_config)?;

        let price_data: Vec<PriceDataRaw> = serde_json::from_str(&price_history_as_json.as_str())?;
        let price_history: Vec<PriceData> = price_data.iter().map(|x| PriceData::new(x)).collect();

        let config: Config = serde_yaml::from_str(&config_as_yaml.as_str())?;
        config.validate_config()?;

        let mut bots = Vec::<Bot>::new();
        for _x in 0..config.number_of_bots {
            bots.push(Bot::new(&config))
        }

        let simulation = Simulation {
            price_history,
            config,
            bots
        };

        Ok(simulation)
    }

    pub fn state(&self) {
        println!("Waiting")
    }
}