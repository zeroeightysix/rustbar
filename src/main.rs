#![feature(box_syntax)]
extern crate gtk_layer_shell_rs as gtk_layer_shell;

use std::{
    env::args,
    vec::Vec,
};
use std::path::Path;

use futures::executor::block_on;
use gio::prelude::*;
use gtk::{ApplicationWindow, Orientation, prelude::*, WidgetExt, CssProvider, StyleContext};
use serde_json::json;
use tokio::task::block_in_place;

use crate::config::Config;

mod modules;
mod config;
mod layout;

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
    let style_path = Path::new("style.css");
    if style_path.exists() {
        let provider = CssProvider::new();
        provider.load_from_file(&gio::File::new_for_path(style_path)).expect("Couldn't load custom style");
        StyleContext::add_provider_for_screen(&gdk::Screen::get_default().expect("Couldn't get default GDK screen"), &provider, 800);
        println!("{}", "Applied custom style sheet!");
    }

    let window = gtk::ApplicationWindow::new(application);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    init_layer_shell(&window, cfg);

    let content = gtk::Box::new(Orientation::Horizontal, 0);
    window.add(&content);

    cfg.layout.initialise_handlers(&content);

    block_in_place(|| { window.show_all() });
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
