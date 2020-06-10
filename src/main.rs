extern crate gtk_layer_shell_rs as gtk_layer_shell;

use std::{
    env::args,
    vec::Vec,
};

use futures::executor::block_on;
use gio::prelude::*;
use gtk::{ApplicationWindow, prelude::*, WidgetExt, Label};
use tokio::time::delay_for;
use chrono::Local;

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

    let date_label = gtk::Label::new(None);
    content_box.add(&date_label);

    let mut idle_functions = Vec::new();
    idle_functions.push(create_date_module(date_label).await);

    window.add(&content_box);

    gtk::idle_add(move || {
        for f in idle_functions.iter_mut() {
            f();
        }

        Continue(true)
    });

    window.show_all();
}

async fn create_date_module(date_label: Label) -> Box<dyn FnMut()> {
    let (mut tx, mut rx) = tokio::sync::mpsc::channel(2);
    tokio::spawn(async move {
        loop {
            let date = Local::now();
            let _ = tx.send(format!("{}", date.format("%H:%M:%S"))).await;
            delay_for(tokio::time::Duration::from_secs(1)).await;
        }
    });

    let idle_func = move || {
        if let Ok(s) = rx.try_recv() {
            date_label.set_text(s.as_str());
        }
    };

    Box::new(idle_func)
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
