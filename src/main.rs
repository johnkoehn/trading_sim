pub mod simulation;
pub mod village;
pub mod asset;
pub mod price_data;
pub mod bot;
pub mod config;
#[macro_use]
extern crate serde_derive;
use crate::simulation::Simulation;
use std::collections::HashMap;
// use crate::bot::Bot;
// use crate::village::Village;
use std::error::Error;
// use std::thread;
// use std::sync::mpsc;
use std::io;
// use std::fs;

fn run_simulation() -> Result<(), Box<dyn Error>> {
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

    // println!("{:?}", &simulation);

    Ok(())
}

fn main() {
    println!("{:?}", asset::Asset::BTC);

    // let mut my_hash = HashMap::new();
    // let string_one = String::from("Test1");
    // let string_two = String::from("Test2");
    // my_hash.insert(0, &string_one);
    // my_hash.insert(1, &string_one);
    // my_hash.insert(2, &string_two);
    // my_hash.insert(3, &string_two);

    // let string_three = my_hash.get(&0).unwrap();
    // let string_four = my_hash.get(&1).unwrap();
    // println!("{}", my_hash.get(&0).unwrap());
    // println!("{}", my_hash.get(&1).unwrap());
    // println!("{}", string_three);

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        run_simulation().unwrap();
    }
    // simulation.unwrap();
    // println!("{:?}", simulation);

    // Change terminal to run in a loop. When user hits enter, create simulation
    // If simulation fails to create, show error and have user hit enter to attempt creating again
}
