extern crate gtk_layer_shell_rs as gtk_layer_shell;

use std::{
    env::args,
    vec::Vec,
};

use gio::prelude::*;
use gtk::{
    prelude::*,
    WidgetExt,
};

use modules::module::*;

pub mod modules {
    pub mod module;
}

// upgrade weak reference or return
#[macro_export]
macro_rules! upgrade_weak {
    ($x:ident, $r:expr) => {{
        match $x.upgrade() {
            Some(o) => o,
            None => return $r,
        }
    }};
    ($x:ident) => {
        upgrade_weak!($x, ())
    };
}

fn main() {
    let application = gtk::Application::new(Some("me.zeroeightsix.rustbar"), Default::default())
        .expect("Initialisation failed");

    application.connect_activate(|app| {
        activate(app);
    });

    application.run(&args().collect::<Vec<_>>());
}

fn activate(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk_layer_shell::init_for_window(&window);
    gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Top);
    gtk_layer_shell::auto_exclusive_zone_enable(&window);

    gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Top, 0);
    gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Bottom, 0);
    gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Left, 0);
    gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Right, 0);

    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Top, true);
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Bottom, false);
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Left, true);
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Right, true);

    let content_box = gtk::Box::new(gtk::Orientation::Horizontal, 16);
    content_box.set_halign(gtk::Align::Fill);

    window.add(&content_box);

    window.show_all();
}
