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

use std::env::args;


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

    let label = gtk::Label::new(Some("Hello world!"));
    content_box.add(&label);
    // window.set_border_width(12);

    let date_module = modules::date::create_widget();

    date_module.handle(&"Hello world");

    content_box.add(date_module.get_widget());

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
