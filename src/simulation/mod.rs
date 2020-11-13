use std::vec::Vec;
use std::fs;
use serde_json;
use serde_yaml;
use std::error::Error;
use crate::bot::Bot;
use crate::price_data::{PriceData, PriceDataRaw};
use crate::config::Config;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::Arc;

#[derive(Debug)]
pub struct Simulation {
    price_history: Arc<Vec<PriceData>>,
    config: Arc<Config>,
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
            price_history: Arc::new(price_history),
            config: Arc::new(config),
            bots
        };

        Ok(simulation)
    }

    pub fn run(&self) {
        // start the simulation
        // we will need to play around here to see what our options are for running the simulation
        let (tx, rx): (Sender<Vec<Bot>>, Receiver<Vec<Bot>>) = mpsc::channel();

        let mut children = Vec::new();

        let number_of_bots_per_thread = self.config.number_of_bots / self.config.number_of_threads;

        for thread_number in 0..self.config.number_of_threads {
            let mut bots = Vec::<Bot>::new();
            for _ in 0..number_of_bots_per_thread {
                bots.push(Bot::new(&self.config));
            }

            let price_history = Arc::clone(&self.price_history);
            let config = Arc::clone(&self.config);
            let tx_copy = mpsc::Sender::clone(&tx);
            let child = thread::spawn(move || {
                // we start the simulation at the max number of averaging periods to give all bots a fair shot
                for x in config.traits.number_of_averaging_periods.max..price_history.len() as u64 {
                    bots
                        .iter_mut()
                        .for_each(|bot| bot.run_period(&price_history, x, &config))
                }

                // TODO: We need to force sell of products
                // calculate fitness (do simple calculation)
                // work on breeding
                // after that we can run many generations
                // and finally we can begin graphing data
                // don't forget to properly handle inbreeding

                let result = tx_copy.send(bots);

                match result {
                    Ok(v) => println!("Thread {} finished", thread_number),
                    Err(e) => println!("Error sending message on thread {} with err: {:?}", thread_number, e)
                }
            });

            children.push(child);
        }

        for _ in 0..self.config.number_of_threads {
            rx.recv();
        }
    }

    pub fn state(&self) {
        println!("Waiting")
    }
}