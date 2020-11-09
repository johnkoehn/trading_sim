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
use std::thread;
use std::sync::mpsc;

fn spawn_villages(number_of_villages: u32) -> Vec<Village> {
    // for village in villages
    let mut villages = Vec::new();

    for i in 0..number_of_villages {
        let village = Village::new();
        villages.push(village);
    }

    villages
}

fn main() {
    // let mut simulation = Simulation::new();
    // println!("Hello, world!");
    // simulation.state();

    // let mut villages = spawn_villages(10);

    // let (tx, rx) = mpsc::channel();

    // for mut village in villages {
    //     let tx_copy = mpsc::Sender::clone(&tx);
    //     thread::spawn(move || {
    //         village.update();
    //         tx_copy.send(village).unwrap();
    //     });
    // }

    // for updated_village in rx {
    //     println!("Updated village: {:?}", updated_village)
    // }

    // can a main thread send messages back to those threads?
    // imagine reciving trades
    // executing the trades
    // and sending the trade data back ?

    println!("{:?}", asset::Asset::BTC);

    let bot = Bot::new();

    let simulation = Simulation::new("./historicalData/etherumPriceData.json", "./config/config.yaml");

    println!("{:?}", simulation);

    // Change terminal to run in a loop. When user hits enter, create simulation
    // If simulation fails to create, show error and have user hit enter to attempt creating again
}
