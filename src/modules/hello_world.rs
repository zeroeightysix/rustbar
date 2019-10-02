extern crate glib;
extern crate gtk;
extern crate serde;
extern crate serde_json;

use super::module::Module;
use std::{
    thread,
    time::Duration,
};
use serde::Deserialize;
use glib::Sender;
use gtk::{
    Label,
    LabelExt,
};

#[derive(Deserialize, Debug, PartialEq)]
struct ConfigExtra {
    wait: Option<usize>,
}

pub fn create_module<'a>(tx: Sender<String>, extra: Option<serde_json::Value>) -> Module<'a, Label, String> {
    let mut wait = 2;

    if let Some(json_value) = extra {
        match serde_json::from_value::<ConfigExtra>(json_value) {
            Ok(extra) => {
                if let Some(extra_wait) = extra.wait {
                    wait = extra_wait;
                }
            },
            Err(_) => panic!(),
        }
    }

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(wait as u64));
        tx.send(String::from("Bye world!"))
    });

    Module::new(Label::new(Some("Hello world!")), &|m, msg| {
        m.get_widget().set_text(msg);
    })
}