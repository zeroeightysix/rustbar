extern crate glib;
extern crate serde_json;
extern crate chrono;

use glib::Sender;
use gtk::{Label, ContainerExt};
use std::thread;
use std::time;
use gtk::LabelExt;
use serde::Deserialize;
use serde_json::Value;
use chrono::prelude::*;

pub trait Module {
    fn add_widget(&self, container: &gtk::Box);
}

pub struct DateModule {
    widget: Label
}

#[derive(Deserialize, Debug, PartialEq)]
struct DateExtra {
    format: Option<String>
}

impl DateModule {
    pub fn new(tx: Sender<String>, extra: Option<Value>) -> DateModule {
        let label = gtk::Label::new(Some("date"));
        
        let extra = extra.and_then(|extra| serde_json::from_value::<DateExtra>(extra).ok());
        // Refer to https://docs.rs/chrono/0.4.10/chrono/format/strftime/index.html#specifiers
        let date_format = extra.and_then(|extra| extra.format).unwrap_or(String::from("%Y-%m-%d %H:%M:%S"));

        thread::spawn(move || {
            loop {
                let dt = Local::now();
                let text = dt.format(date_format.as_ref()).to_string();
                match tx.send(text) {
                    Ok(()) => (),
                    Err(e) => panic!(e)
                }
                thread::sleep(time::Duration::new(1, 0));
            }
        });
        DateModule {
            widget: label
        }
    }

    pub fn handle_message(module: &DateModule, message: String) {
        module.widget.set_text(message.as_ref());
    }
}

impl Module for DateModule {
    fn add_widget(&self, container: &gtk::Box) {
        container.add(&self.widget);
    }
}