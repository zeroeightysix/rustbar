extern crate serde;
extern crate json5;

use std::{
    io::prelude::*,
    fs::File,
};
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ConfigAlign {
    Fill,
    Start,
    End,
    Center,
    Baseline
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ConfigModule {
    name: String,
    align: Option<ConfigAlign>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Config {
    modules: Vec<ConfigModule>,
}

impl Config {
    fn read_file(path: &str) -> Result<String, std::io::Error> {
        match File::open(path) {
            Ok(mut file) => {
                let mut content = String::new();
                match file.read_to_string(&mut content) {
                    Ok(_) => Ok(content),
                    Err(e) => Err(e)
                }
            },
            Err(e) => Err(e)
        }
    }

    pub fn from_file(path: &str) -> Result<Config, String> {
        let content = match Config::read_file(path) {
            Ok(content) => content,
            Err(e) => return Err(e.to_string()),
        };
        match json5::from_str(&content) {
            Ok(config) => Ok(config),
            Err(e) => Err(e.to_string())
        }
    }
}