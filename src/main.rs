extern crate gtk_layer_shell_rs as gtk_layer_shell;

use std::{
    env::args,
    vec::Vec,
};

use futures::executor::block_on;
use gio::prelude::*;
use gtk::{ApplicationWindow, prelude::*, WidgetExt};
use tokio::time::delay_for;

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

    let date_label = gtk::Label::new(Some("Hello world!"));

    let (mut tx, mut rx) = tokio::sync::mpsc::channel(10);
    tokio::spawn(async move {
        let mut c = 0;
        loop {
            delay_for(tokio::time::Duration::from_secs(1)).await;
            c += 1;
            let _ = tx.send(c).await;
        }
    });
    content_box.add(&date_label);

    window.add(&content_box);

    gtk::idle_add(move || {
        if let Ok(t) = rx.try_recv() {
            date_label.set_text(format!("{}", t).as_str());
        }

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
