pub mod simulation;
pub mod village;
pub mod asset;
pub mod price_data;
pub mod bot;
pub mod config;
#[macro_use]
extern crate serde_derive;
use crate::simulation::Simulation;
use std::error::Error;
use std::io;

fn run_simulation() -> Result<(), Box<dyn Error>> {
    // TODO: Handle situations where /simulations/{{dir}} doesn't exist (i.e. create the directory)
    let simulation_result = Simulation::new("./historicalData/etherumPriceData.json", "./config/config.yaml");
    let mut simulation = match simulation_result {
        Ok(simulation) => simulation,
        Err(e) => {
            println!("{}", e.to_string());
            return Err(e)
        }
    };

    println!("Simulation loaded");
    simulation.state();

    simulation.run(1);

    Ok(())
}

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        run_simulation().unwrap();
    }
}
