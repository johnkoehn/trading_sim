pub mod simulation;
pub mod village;
pub mod asset;
pub mod price_data;
pub mod bot;
pub mod config;
#[macro_use]
extern crate serde_derive;
use crate::simulation::Simulation;
use crate::bot::Bot;
use crate::village::Village;
use std::error::Error;
use std::thread;
use std::sync::mpsc;
use std::io;

fn run_simulation() -> Result<(), Box<dyn Error>> {
    let simulation_result = Simulation::new("./historicalData/etherumPriceData.json", "./config/config.yaml");
    let simulation = match simulation_result {
        Ok(simulation) => simulation,
        Err(e) => {
            println!("{}", e.to_string());
            return Err(e)
        }
    };

    println!("Simulation loaded");
    simulation.state();

    // println!("{:?}", &simulation);

    Ok(())
}

fn main() {
    println!("{:?}", asset::Asset::BTC);

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        run_simulation();
    }
    // simulation.unwrap();
    // println!("{:?}", simulation);

    // Change terminal to run in a loop. When user hits enter, create simulation
    // If simulation fails to create, show error and have user hit enter to attempt creating again
}
