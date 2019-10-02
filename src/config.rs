extern crate serde;
extern crate json5;
extern crate gtk;

use std::{
    io::prelude::*,
    fs::File,
};
use gtk::Align;
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

impl ConfigAlign {
    pub fn to_gtk(&self) -> Align {
        match self {
            ConfigAlign::Fill => gtk::Align::Fill,
            ConfigAlign::Start => gtk::Align::Start,
            ConfigAlign::End => gtk::Align::End,
            ConfigAlign::Center => gtk::Align::Center,
            ConfigAlign::Baseline => gtk::Align::Baseline,
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ConfigModule {
    pub name: String,
    pub align: Option<ConfigAlign>,
    pub expand: Option<bool>,
    pub margin_start: Option<i32>,
    pub margin_end: Option<i32>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Config {
    pub modules: Vec<ConfigModule>,
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