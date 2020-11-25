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
use actix_cors::Cors;
use actix_web::{get, post, put, web, App, HttpResponse, HttpServer, Responder};

#[derive(Debug, Serialize, Deserialize)]
struct SimulationResponse {
    id: String
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Healthy")
}

#[get("/configs/{config_name}")]
async fn get_config(web::Path(config_name): web::Path<(String)>) -> impl Responder {
    let file_path = "./configs/".to_owned() + config_name.as_str() + ".yaml";

    let config_as_yaml_result = fs::read_to_string(file_path);
    if config_as_yaml_result.is_err() {
        return HttpResponse::NotFound()
            .body("")
    }

    let config_as_yaml = config_as_yaml_result.unwrap();
    let config: Config = serde_yaml::from_str(&config_as_yaml.as_str()).unwrap();
    let config_as_json = serde_json::to_string(&config).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(&config_as_json)
}

#[get("/configs")]
async fn list_configs() -> impl Responder {
    let entries = fs::read_dir("./configs").unwrap();

    let mut config_names = Vec::<String>::new();
    for entry in entries {
        let directory_buffer = entry.unwrap().path();
        let file_path = directory_buffer.to_str().unwrap();

        let directory_regex = Regex::new("./configs\\\\(.*).yaml").unwrap();
        let captures = directory_regex.captures(file_path).unwrap();
        let config_name = &captures[1];

        config_names.push(String::from(config_name));
    }

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&config_names).unwrap())
}

#[put("/configs/{config_name}")]
async fn create_or_update_config(config_web_request: web::Json<Config>, web::Path(config_name): web::Path<(String)>) -> impl Responder {
    let config = config_web_request.into_inner();
    let file_path = "./configs/".to_owned() + config_name.as_str() + ".yaml";

    let config_as_yaml = serde_yaml::to_string(&config).unwrap();
    fs::write(file_path, config_as_yaml).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&config).unwrap())
}

#[get("/configs/form")]
async fn get_config_form() -> impl Responder {
    let config_form_as_yaml = fs::read_to_string("./config_form.yaml").unwrap();
    let config_form: serde_yaml::Value = serde_yaml::from_str(&config_form_as_yaml.as_str()).unwrap();
    let config_as_json = serde_json::to_string(&config_form).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(&config_as_json)
}

#[post("/configs/validate")]
async fn validate_config(config: web::Json<Config>) -> impl Responder {
    let config_errors = config.into_inner().validate_config();

    if config_errors.len() > 0 {
        return HttpResponse::BadRequest()
            .content_type("application/json")
            .body(serde_json::to_string(&config_errors).unwrap());
    }

    return HttpResponse::Ok()
        .body("")
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
    let entries = fs::read_dir("./simulations").unwrap();

    let mut ids = Vec::<String>::new();
    for entry in entries {
        let directory_buffer = entry.unwrap().path();
        let directory_as_str = directory_buffer.to_str().unwrap();

        let directory_regex = Regex::new("./simulations\\\\").unwrap();
        let sim_id_result = directory_regex.replace_all(directory_as_str, "");

        let id = sim_id_result.into_owned();
        ids.push(id);
    }


    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&ids).unwrap())
}

#[get("/simulations/{simulation_id}/generations")]
async fn list_generations(web::Path(simulation_id): web::Path<(String)>) -> impl Responder {
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
    let generation_path = "./simulations/".to_owned() + simulation_id.as_str() + "/" + generation_id.as_str();
    let generation = fs::read_to_string(generation_path).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(generation)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .service(health)
            .service(get_config_form)
            .service(get_config)
            .service(list_configs)
            .service(create_or_update_config)
            .service(validate_config)
            .service(start_simulation)
            .service(list_simulations)
            .service(list_generations)
            .service(get_generation)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
