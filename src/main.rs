pub mod simulation;
pub mod village;
pub mod asset;
pub mod price_data;
pub mod bot;
pub mod config;
use config::Config;
use regex::Regex;
#[macro_use]
extern crate serde_derive;
extern crate alphanumeric_sort;
use crate::simulation::Simulation;
use std::thread;
use std::fs;
use serde::{Deserialize, Serialize};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[derive(Debug, Serialize, Deserialize)]
struct SimulationResponse {
    id: String
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Healthy")
}

#[get("/configs/default")]
async fn get_config() -> impl Responder {
    let config_as_yaml = fs::read_to_string("./config/config.yaml").unwrap();
    let config: Config = serde_yaml::from_str(&config_as_yaml.as_str()).unwrap();
    let config_as_json = serde_json::to_string(&config).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(&config_as_json)
}

#[post("/simulations")]
async fn start_simulation(config: web::Json<Config>) -> impl Responder {
    let mut simulation = Simulation::web_create("./historicalData/etherumPriceData.json", config.into_inner()).unwrap();
    thread::spawn(move || {
        simulation.run(1);
    });

    let simulation_response = SimulationResponse {
        id: String::from("current")
    };

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&simulation_response).unwrap())
}

#[get("/simulations")]
async fn list_simulations() -> impl Responder {
    // read in simulations directory
    let entries = fs::read_dir("./simulations").unwrap();

    let mut ids = Vec::<String>::new();
    for entry in entries {
        let directory_buffer = entry.unwrap().path();
        let directort_as_str = directory_buffer.to_str().unwrap();

        let directory_regex = Regex::new("./simulations\\\\").unwrap();
        let sim_id_result = directory_regex.replace_all(directort_as_str, "");

        let id = sim_id_result.into_owned();
        ids.push(id);
    }


    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&ids).unwrap())
}

#[get("/simulations/{simulation_id}/generations")]
async fn list_generations(web::Path(simulation_id): web::Path<(String)>) -> impl Responder {
    // read in simulations directory
    let directory = "./simulations/".to_owned() + simulation_id.as_str();
    let entries = fs::read_dir(&directory).unwrap();

    let mut ids = Vec::<String>::new();
    for entry in entries {
        let directory_buffer = entry.unwrap().path();
        let directort_as_str = directory_buffer.to_str().unwrap();

        let regex_as_str = directory.clone() + "\\\\".to_owned().as_str();
        let directory_regex = Regex::new(regex_as_str.as_str()).unwrap();
        let gen_id_result = directory_regex.replace_all(directort_as_str, "");

        let id = gen_id_result.into_owned();
        ids.push(id);
    }

    alphanumeric_sort::sort_str_slice(&mut ids);

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&ids).unwrap())
}

#[get("/simulations/{simulation_id}/generations/{generation_id}")]
async fn get_generation(web::Path((simulation_id, generation_id)): web::Path<(String, String)>) -> impl Responder {
    // read in simulations directory
    let generation_path = "./simulations/".to_owned() + simulation_id.as_str() + "/" + generation_id.as_str();
    let generation = fs::read_to_string(generation_path).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(generation)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health)
            .service(get_config)
            .service(start_simulation)
            .service(list_simulations)
            .service(list_generations)
            .service(get_generation)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
