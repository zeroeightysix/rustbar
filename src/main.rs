mod modules {
    pub mod date;
    pub mod module;
    pub mod hello_world;
}
pub mod config;

extern crate gtk;
extern crate gio;
extern crate gtk_layer_shell_rs as gtk_layer_shell;
extern crate serde;
extern crate json5;

use gio::prelude::*;
use gtk::{
    prelude::*,
};
use std::{
    vec::Vec,
    env::args
};
use config::Config;

fn activate(application: &gtk::Application) {

    let window = gtk::ApplicationWindow::new(application);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk_layer_shell::init_for_window(&window);
    gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Top);
    gtk_layer_shell::auto_exclusive_zone_enable(&window);

    // gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Bottom, 20);

    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Left, true);
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Right, true);
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Top, true);

    let content_box = gtk::Box::new(gtk::Orientation::Horizontal, 16);

    let c: Config = match Config::from_file("config.json5") {
        Ok(c) => c,
        Err(e) => panic!(e),
    };

    println!("{:?}", c);

    let mut module_names = Vec::new();
    module_names.push("date");
    module_names.push("hello_world");

    for module_name in module_names {
        // Create a receiver and sender for this module.
        // The sender is given to the module. It is free to create a thread that sends to this sender at any time.
        // Because GTK is not thread-safe, the module cannot modify its widget(s) on the seperate thread.
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        let module = match module_name {
            "date" => modules::date::create_module(tx),
            "hello_world" => modules::hello_world::create_module(tx),
            _ => {
                println!("Skipping unknown module {}.", module_name);
                continue
            },
        };

        content_box.add(module.get_widget());

        // If we receive anything from the receiver we just made, pass it back to the module.
        // It can then handle this message on the GTK main thread (this thread), thus is able to modify the widget(s) it made.
        rx.attach(None, move |message| {
            module.handle(&message);
            glib::Continue(true)
        });
    }

    window.add(&content_box);

    window.show_all();

}

fn main() {

    let application = gtk::Application::new(Some("me.zeroeightsix.rustbar"), Default::default())
        .expect("Initialisation failed");

    application.connect_activate(|app| {
        activate(app);
    });
    
    application.run(&args().collect::<Vec<_>>());

}
