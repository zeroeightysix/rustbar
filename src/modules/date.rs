extern crate gio;
extern crate gtk;
extern crate glib;

use std::{
    thread,
    time
};
use gtk::{
    Label,
    LabelExt
};
use glib::Sender;
use super::module::Module;

pub fn create_widget<'a>(tx: Sender<String>) -> Module<'a, Label, String> {

    let label = Label::new(Some("date"));

    let _updater = thread::spawn(move || {
        thread::sleep(time::Duration::from_secs(1));
        tx.send(String::from("Hello world"))
    });

    Module::new(label, &|m, msg| {
        m.get_widget().set_text(msg);
        println!("{}", msg)
    })

}