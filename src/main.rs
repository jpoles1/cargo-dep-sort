#![feature(preserve_order)]

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
    let toml_raw = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    println!("Loaded in TOML data:\n{}", toml_raw);
    let toml_data : toml::Value = toml::de::from_str(&toml_raw).unwrap();
    println!("TOML parsed:\n{:?}", toml_data);
    let toml_clean : String = toml::ser::to_string_pretty(&toml_data).unwrap();
    println!("TOML cleaned:\n{}", toml_clean);
}

fn main() {
    println!("Hello, world!");
    load_toml("test_data/rustlings.toml")
}

