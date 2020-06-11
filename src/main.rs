#![feature(box_syntax)]
extern crate gtk_layer_shell_rs as gtk_layer_shell;

use std::{
    env::args,
    vec::Vec,
};
use std::path::Path;

use futures::executor::block_on;
use gio::prelude::*;
use gtk::{ApplicationWindow, prelude::*, WidgetExt};
use serde_json::json;

use crate::{
    config::Config,
    modules::{
        date::DateModule,
        module::Module,
    },
    modules::hello::HelloModule,
};
use crate::modules::workspace::WorkspaceModule;

mod modules;
mod config;

macro_rules! add_module {
    (
        $nm:expr,
        $cb:expr,
        $fn:expr,
        $js:expr,
        $(
            $name:expr => $m:ident
        );*
    ) => {
        $(
            if $nm == $name {
                let (f, w) = $m::from_value($js).into_widget_handler().await;
                $fn.push(f);
                $cb.add(&w);
            }
        )*
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let cfg_path = Path::new("config.json5");
    let cfg = if !cfg_path.exists() {
        serde_json::from_value(json!({}))?
    } else {
        json5::from_str::<Config>(std::fs::read_to_string(cfg_path).unwrap().as_str()).unwrap()
    };

    let application = gtk::Application::new(Some("me.zeroeightsix.rustbar"), Default::default())
        .expect("Initialisation failed");

    application.connect_activate(move |app| {
        block_on(activate(app, &cfg));
    });

    application.run(&args().collect::<Vec<_>>());

    Ok(())
}

async fn activate(application: &gtk::Application, cfg: &Config) {
    let window = gtk::ApplicationWindow::new(application);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    init_layer_shell(&window, cfg);

    let content_box = gtk::Box::new(gtk::Orientation::Horizontal, 16);
    content_box.set_halign(gtk::Align::Fill);
    window.add(&content_box);

    let mut idle_functions = Vec::new();

    for v in &cfg.modules {
        let name = &v["name"];
        if let Some(name) = name.as_str() {
            // We use a macro here because the module is of varying type.
            add_module!(name, content_box, idle_functions, v,
                "date" => DateModule;
                "hello" => HelloModule;
                "workspaces" => WorkspaceModule
            );
        }
    }

    // GTK is non thread-safe, so all modules get a chance to do something on the main thread here.
    // Thus, it is expected that all modules only ever modify their widgets through the handler functions,
    // and perform other asynchronous things using channels.
    gtk::idle_add(move || {
        idle_functions.iter_mut().for_each(|f| f());

        Continue(true)
    });

    window.show_all();
}

/// Initialises the window as a top-level layer shell window. Layer-shell is the protocol
/// used for things like docks, notification windows, bars(!), etc.
fn init_layer_shell(window: &ApplicationWindow, cfg: &Config) {
    gtk_layer_shell::init_for_window(window);
    gtk_layer_shell::set_layer(window, gtk_layer_shell::Layer::Top);
    gtk_layer_shell::auto_exclusive_zone_enable(window);

    gtk_layer_shell::set_margin(window, gtk_layer_shell::Edge::Top, cfg.margins.top);
    gtk_layer_shell::set_margin(window, gtk_layer_shell::Edge::Bottom, cfg.margins.bottom);
    gtk_layer_shell::set_margin(window, gtk_layer_shell::Edge::Left, cfg.margins.left);
    gtk_layer_shell::set_margin(window, gtk_layer_shell::Edge::Right, cfg.margins.right);

    gtk_layer_shell::set_anchor(window, gtk_layer_shell::Edge::Top, cfg.anchors.top);
    gtk_layer_shell::set_anchor(window, gtk_layer_shell::Edge::Bottom, cfg.anchors.bottom);
    gtk_layer_shell::set_anchor(window, gtk_layer_shell::Edge::Left, cfg.anchors.left);
    gtk_layer_shell::set_anchor(window, gtk_layer_shell::Edge::Right, cfg.anchors.right);
}
