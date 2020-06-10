extern crate gtk_layer_shell_rs as gtk_layer_shell;

use std::{
    env::args,
    vec::Vec,
};

use futures::executor::block_on;
use gio::prelude::*;
use gtk::{ApplicationWindow, prelude::*, WidgetExt};

use crate::{
    modules::{
        date::DateModule,
        module::Module,
    }
};

mod modules;

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

#[tokio::main]
async fn main() {
    let application = gtk::Application::new(Some("me.zeroeightsix.rustbar"), Default::default())
        .expect("Initialisation failed");

    application.connect_activate(|app| {
        block_on(activate(app));
    });

    application.run(&args().collect::<Vec<_>>());
}

async fn activate(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    init_layer_shell(&window);

    let content_box = gtk::Box::new(gtk::Orientation::Horizontal, 16);
    content_box.set_halign(gtk::Align::Fill);

    let mut idle_functions = Vec::new();
    let (f, w) = DateModule::create_module().await;
    idle_functions.push(f);
    content_box.add(&w);

    window.add(&content_box);

    gtk::idle_add(move || {
        idle_functions.iter_mut().for_each(|f| f());

        Continue(true)
    });

    window.show_all();
}

fn init_layer_shell(window: &ApplicationWindow) {
    gtk_layer_shell::init_for_window(window);
    gtk_layer_shell::set_layer(window, gtk_layer_shell::Layer::Top);
    gtk_layer_shell::auto_exclusive_zone_enable(window);

    gtk_layer_shell::set_margin(window, gtk_layer_shell::Edge::Top, 0);
    gtk_layer_shell::set_margin(window, gtk_layer_shell::Edge::Bottom, 0);
    gtk_layer_shell::set_margin(window, gtk_layer_shell::Edge::Left, 0);
    gtk_layer_shell::set_margin(window, gtk_layer_shell::Edge::Right, 0);

    gtk_layer_shell::set_anchor(window, gtk_layer_shell::Edge::Top, true);
    gtk_layer_shell::set_anchor(window, gtk_layer_shell::Edge::Bottom, false);
    gtk_layer_shell::set_anchor(window, gtk_layer_shell::Edge::Left, true);
    gtk_layer_shell::set_anchor(window, gtk_layer_shell::Edge::Right, true);
}
