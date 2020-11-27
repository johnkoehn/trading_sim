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
use std::collections::HashMap;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Simulation {
    price_history: Arc<Vec<PriceData>>,
    config: Arc<Config>,
    bots: Vec<Bot>,
    id: String
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
enum Status {
    RUNNING,
    FAILED,
    COMPLETED
}

#[derive(Debug, Serialize, Deserialize)]
struct SimulationStatus {
    status: Status
}

pub fn breed(bots: &Vec::<Bot>, config: &Config) -> Vec::<Bot> {
    // caculate total fitness
    let total_fitness: f64 = bots.iter()
        .map(|bot| bot.fitness)
        .sum();

    let mut breeding_pool: HashMap<u64, &Bot> = HashMap::new();

    for bot in bots {
        // calculate number of tickets
        // always round up
        // we may have over 100 breeding tickets -- that's okay :}

        let fitness = bot.fitness;
        if fitness > 0.0 {
            let percent_of_fitness = (fitness / total_fitness) * 100.0;
            let number_of_tickets = percent_of_fitness.ceil() as u64;

            for _ in 0..number_of_tickets {
                let position = breeding_pool.len() as u64;
                breeding_pool.insert(position, &bot);
            }
        }
    }

    let mut new_bots = Vec::<Bot>::new();
    let mut rng = rand::thread_rng();

    while new_bots.len() < config.number_of_bots as usize {
        let index_one = rng.gen_range(0, breeding_pool.len() - 1) as u64;

        let bot_one = breeding_pool.get(&index_one).unwrap();
        loop {
            let index_two = rng.gen_range(0, breeding_pool.len() - 1) as u64;
            let bot_two = breeding_pool.get(&index_two).unwrap();

            if bot_one.id != bot_two.id {
                // breed
                let baby_bot = bot_one.breed(&bot_two, &mut rng, config, new_bots.len() as u64);
                new_bots.push(baby_bot);
                break;
            }
        }
    }

    new_bots
}

// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
impl Simulation {
    pub fn web_create(path_to_price_history: &str, config: Config, id: String) -> Result<Simulation, Box<dyn Error>> {
        let price_history_as_json = fs::read_to_string(path_to_price_history)?;

        let price_data: Vec<PriceDataRaw> = serde_json::from_str(&price_history_as_json.as_str())?;
        let price_history: Vec<PriceData> = price_data.iter().map(|x| PriceData::new(x)).collect();

        if config.validate_config().len() > 0 {
            panic!("Config validation failed!")
        }

        let mut bots = Vec::<Bot>::new();
        for id in 0..config.number_of_bots {
            bots.push(Bot::new(&config, id));
        }

        let simulation = Simulation {
            price_history: Arc::new(price_history),
            config: Arc::new(config),
            bots,
            id
        };

        Ok(simulation)
    }


    fn update_status(&self, simulation_status: &SimulationStatus) {
        let simulation_status_as_json = serde_json::to_string_pretty(simulation_status).unwrap();

        let file_name = format!("./simulations/{}/status.json", self.id);
        fs::write(file_name, simulation_status_as_json).unwrap();
    }

    // create a function to start the simulation
    // this function will manage the state
    // any errors will result in setting state to failed
    pub fn start_simulation(&mut self) {
        let path = format!("./simulations/{}/results", self.id);
        fs::create_dir_all(&path).unwrap();

        let mut simulation_status = SimulationStatus {
            status: Status::RUNNING
        };
        self.update_status(&simulation_status);

        let run_result = self.run(1);

        if run_result.is_err() {
            simulation_status.status = Status::FAILED;
            self.update_status(&simulation_status);
            return;
        }

        simulation_status.status = Status::COMPLETED;
        self.update_status(&simulation_status);
    }


    pub fn run(&mut self, generation: u64) -> Result<(), Box<dyn Error>> {
        if generation > self.config.number_of_generations {
            return Ok(());
        }

        println!("Generation {}", generation);

        // start the simulation
        // we will need to play around here to see what our options are for running the simulation
        let (tx, rx): (Sender<Vec<Bot>>, Receiver<Vec<Bot>>) = mpsc::channel();

        let mut children = Vec::new();

        let number_of_bots_per_thread = self.config.number_of_bots / self.config.number_of_threads;

        for thread_number in 0..self.config.number_of_threads {
            let mut bots = Vec::<Bot>::new();
            for _x in 0..number_of_bots_per_thread {
                let bot = self.bots.pop().unwrap();

                bots.push(bot);
            }

            let price_history = Arc::clone(&self.price_history);
            let config = Arc::clone(&self.config);
            let tx_copy = mpsc::Sender::clone(&tx);
            let child = thread::spawn(move || {
                // we start the simulation at the max number of averaging periods to give all bots a fair shot
                // for x in config.traits.number_of_averaging_periods.max..price_history.len() as u64 {
                for x in 0..price_history.len() as u64 {
                    bots
                        .iter_mut()
                        .for_each(|bot| bot.run_period(&price_history, x, &config))
                }

                // calculate fitness (do simple calculation)
                // work on breeding
                // after that we can run many generations
                // and finally we can begin graphing data
                // don't forget to properly handle inbreeding
                // for bot in &bots {
                //     println!("{:?}", bot.money);
                // }
                let result = tx_copy.send(bots);

                match result {
                    Ok(_v) => println!("Thread {} finished", thread_number),
                    Err(e) => println!("Error sending message on thread {} with err: {:?}", thread_number, e)
                }
            });

            children.push(child);
        }

        let mut bots_post_simulation = Vec::<Bot>::new();
        for _ in 0..self.config.number_of_threads {
            let mut bots = rx.recv()?;
            bots_post_simulation.append(&mut bots);
        }

        // sort the bots by fitness
        bots_post_simulation.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        for bot in &bots_post_simulation {
            println!("{:?}", bot.money);
        }

        // write the bots to a generations file
        let results_as_json = serde_json::to_string_pretty(&bots_post_simulation)?;

        let file_name = format!("./simulations/{}/results/generation_{}.json", self.id, generation);
        fs::write(file_name, results_as_json)?;

        let next_generation_bots = breed(&bots_post_simulation, &self.config);
        self.bots = next_generation_bots;
        self.run(generation + 1);

        return Ok(())
    }

    pub fn state(&self) {
        println!("Waiting")
    }
}