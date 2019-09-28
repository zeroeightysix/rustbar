extern crate gio;
extern crate gtk;
extern crate glib;

use gtk::Label;
use super::module::Module;

pub fn create_widget<'a>() -> Module<'a, Label, &'a str> {

    Module::new(Label::new(Some("date")), &|msg| println!("{}", msg))

}