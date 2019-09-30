extern crate glib;
extern crate gtk;

use super::module::Module;
use std::{
    thread,
    time::Duration,
};
use glib::Sender;
use gtk::{
    Label,
    LabelExt,
};

pub fn create_module<'a>(tx: Sender<String>) -> Module<'a, Label, String> {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        tx.send(String::from("Bye world!"))
    });

    Module::new(Label::new(Some("Hello world!")), &|m, msg| {
        m.get_widget().set_text(msg);
    })
}