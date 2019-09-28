extern crate gio;
extern crate gtk;
extern crate glib;

use gtk::{
    Label,
    LabelExt
};
use super::module::Module;

pub fn create_widget<'a>() -> Module<'a, Label, &'a str> {

    let label = Label::new(Some("date"));

    Module::new(label, &|m, msg| {
        m.get_widget().set_text(msg);
        println!("{}", msg)
    })

}