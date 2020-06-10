extern crate gtk_layer_shell_rs as gtk_layer_shell;

use std::{
    env::args,
    fs::File,
    vec::Vec,
};
use std::path::Path;

use futures::executor::block_on;
use gio::prelude::*;
use gtk::{ApplicationWindow, prelude::*, WidgetExt};
use serde_json::json;

use crate::{
    modules::{
        date::DateModule,
        module::Module,
    }
};
use crate::config::Config;
use crate::modules::hello::HelloModule;

mod modules;
mod config;

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

macro_rules! add_module {
    (
        $cb:expr,
        $fn:expr,
        $in:expr,
        $(
            $name:expr => $m:ident
        );*
    ) => {
        $(
            if $in == $name {
                let (f, w) = $m::new().into_widget_handler().await;
                $fn.push(f);
                $cb.add(&w);
            }
        )*
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let cfg_path = Path::new("config.json");
    let cfg = if !cfg_path.exists() {
        serde_json::from_value(json!({}))
    } else {
        serde_json::from_reader(File::open(cfg_path)?)
    }?;

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

    let mut idle_functions = Vec::new();

    for v in &cfg.modules {
        let name = &v["name"];
        if let Some(name) = name.as_str() {
            add_module!(content_box, idle_functions, name,
                "date" => DateModule;
                "hello" => HelloModule
            );
        }
    }

    window.add(&content_box);

    gtk::idle_add(move || {
        idle_functions.iter_mut().for_each(|f| f());

        Continue(true)
    });

    window.show_all();
}

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
