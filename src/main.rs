pub mod modules {
    pub mod module;
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
    WidgetExt,
};
use std::{
    vec::Vec,
    env::args
};
use config::Config;
use modules::module::*;

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

fn unpack_extra<T>(extra: Option<serde_json::Value>) -> Option<T>
where for<'de> T: serde::Deserialize<'de>
{
    match extra {
        Some(value) => match serde_json::from_value(value) {
            Ok(extra) => Some(extra),
            Err(_) => None
        },
        None => None
    }
}

fn activate(application: &gtk::Application) {

    let c: Config = match Config::from_path("config.json5") {
        Ok(c) => c,
        Err(e) => panic!(e),
    };

    let window = gtk::ApplicationWindow::new(application);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk_layer_shell::init_for_window(&window);
    gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Top);
    gtk_layer_shell::auto_exclusive_zone_enable(&window);

    let margins = c.margins.unwrap_or(config::ConfigMargins {
        top: Some(0),
        bottom: Some(0),
        left: Some(0),
        right: Some(0),
    });

    gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Top, margins.top.unwrap_or(0));
    gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Bottom, margins.bottom.unwrap_or(0));
    gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Left, margins.left.unwrap_or(0));
    gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Right, margins.right.unwrap_or(0));

    let anchors = c.anchors.unwrap_or(config::ConfigAnchors {
        top: Some(true),
        bottom: Some(false),
        left: Some(true),
        right: Some(true),
    });

    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Top, anchors.top.unwrap_or(true));
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Bottom, anchors.bottom.unwrap_or(false));
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Left, anchors.left.unwrap_or(true));
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Right, anchors.right.unwrap_or(true));

    let content_box = gtk::Box::new(gtk::Orientation::Horizontal, 16);
    content_box.set_halign(gtk::Align::Fill);

    for config_module in c.modules {
        let module_name = config_module.name;
        let extra = config_module.extra;

        // Create a receiver and sender for this module.
        // The sender is given to the module. It is free to create a thread that sends to this sender at any time.
        // Because GTK is not thread-safe, the module cannot modify its widget(s) on the seperate thread.

        match module_name.as_ref() {
            "date" => {
                let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
                let module = DateModule::new(tx);
                module.add_widget(&content_box);
                rx.attach(None, move |message: String| {
                    DateModule::handle_message(&module, message);
                    glib::Continue(true)
                });
            }
            m => {
                println!("Skipping unknown module {}", m);
            }
        }
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
