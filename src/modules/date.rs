extern crate gio;
extern crate gtk;
extern crate glib;
extern crate chrono;
extern crate serde;

use std::{
    thread,
    time
};
use gtk::{
    Label,
    LabelExt
};
use chrono::Local;
use glib::Sender;
use super::module::Module;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct ConfigExtra {
    format: Option<String>,
}

pub fn create_module<'a>(tx: Sender<String>, extra: Option<ConfigExtra>) -> Module<'a, Label, String> {
    let label = Label::new(Some("date"));

    let mut format = String::from("%c");
    if let Some(extra) = extra {
        if let Some(extra_format) = extra.format {
            format = extra_format;
        }
    }

    thread::spawn(move || {
        loop {
            let dt = Local::now().format(format.as_ref()).to_string();
            match tx.send(dt) {
                Ok(_) => (),
                Err(e) => panic!(e),
            }
            thread::sleep(time::Duration::from_secs(1));
        }
    });

    Module::new(label, &|m, msg| {
        m.get_widget().set_text(msg);
    })

}