extern crate gio;
extern crate gtk;
extern crate glib;
extern crate chrono;

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

pub fn create_module<'a>(tx: Sender<String>) -> Module<'a, Label, String> {

    let label = Label::new(Some("date"));

    thread::spawn(move || {
        loop {
            let dt = Local::now();
            let dt = dt.format("%c").to_string();
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