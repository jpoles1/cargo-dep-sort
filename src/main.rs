extern crate toml;
extern crate serde;

use std::env;
use std::fs;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Debug)]
pub struct CargoToml {
    dependencies: toml::value::Table
}

fn load_toml(filename: &str){
    let toml_data = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    println!("Loaded in TOML data:\n{}", toml_data);
    let toml_res : CargoToml = toml::de::from_str(&toml_data).unwrap();
    println!("TOML parsed:\n{:?}", toml_res)
}

fn main() {
    println!("Hello, world!");
    load_toml("test_data/rustlings.toml")
}

