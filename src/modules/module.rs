extern crate glib;

use glib::Sender;
use gtk::{Label, ContainerExt};
use std::thread;
use std::time;
use gtk::LabelExt;

pub trait Module {
    fn add_widget(&self, container: &gtk::Box);
}

pub struct DateModule {
    widget: Label
}

impl DateModule {
    pub fn new(tx: Sender<String>) -> DateModule {
        let label = gtk::Label::new(Some("date"));
        thread::spawn(move || {
            thread::sleep(time::Duration::new(5, 0));
            tx.send(String::from("Hello world!"))
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