mod modules {
    pub mod date;
    pub mod module;
}

extern crate gtk;
extern crate gio;
extern crate gtk_layer_shell_rs as gtk_layer_shell;

use gio::prelude::*;
use gtk::{
    prelude::*,
};
use std::{
    vec::Vec,
    env::args
};

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

    let mut module_names = Vec::new();
    module_names.push("date");

    for module_name in module_names {
        // Create a receiver and sender for this module.
        // The sender is given to the module. It is free to create a thread that sends to this sender at any time.
        // Because GTK is not thread-safe, the module cannot modify its widget(s) on the seperate thread.
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        let module = match module_name {
            "date" => Some(modules::date::create_widget(tx)),
            _ => None,
        };

        if module.is_none() {
            continue;
        };

        let module = module.unwrap();

        content_box.add(module.get_widget());

        // If we receive anything from the receiver we just made, pass it back to the module.
        // It can then handle this message on the GTK main thread (this thread), thus is able to modify the widget(s) it made.
        rx.attach(None, move |text| {
            module.handle(&text);
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
