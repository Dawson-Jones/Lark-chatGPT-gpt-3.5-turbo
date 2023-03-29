use std::{fs, process::exit};

use serde::Deserialize;

#[derive(Deserialize)]
struct Data {
    config: Config,
}

#[derive(Deserialize)]
struct Config {
    ip: String,
    port: u16,
}

fn main() {
    let filename = "./examples/test.toml";
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could'n read file {}", filename);
            exit(1);
        }
    };

    let data: Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from {}", filename);
            exit(1);
        }
    };

    println!("{}", data.config.ip);
    println!("{}", data.config.port);
}
