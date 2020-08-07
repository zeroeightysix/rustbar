use std::collections::HashMap;

use glib::Continue;
use gtk::{LabelExt, Orientation, ContainerExt, BoxExt, WidgetExt, StyleContextExt};
use ksway::{IpcCommand, IpcEvent};
use serde::Deserialize;
use tokio::task::{block_in_place, spawn_blocking};

use crate::modules::module::Module;

#[derive(Deserialize)]
pub struct WorkspaceModule {
    #[serde(default = "default_zero")]
    spacing: i32
}

fn default_zero() -> i32 { 0 }

#[derive(Deserialize, Debug)]
struct WorkspaceEvent {
    change: String,
    old: Option<Workspace>,
    current: Option<Workspace>,
}

#[derive(Deserialize, Debug)]
struct Workspace {
    name: String,
    focused: bool,
    num: usize,
}

impl Workspace {
    fn as_label(&self) -> gtk::Label {
        let label = gtk::Label::new(Some(self.name.as_str()));
        label
    }
}

impl Module<gtk::Box> for WorkspaceModule {
    fn into_widget(self) -> gtk::Box {
        let content = gtk::Box::new(Orientation::Horizontal, self.spacing);

        let mut wp_map = HashMap::new();

        let mut sway = match ksway::client::Client::connect() {
            Ok(client) => client,
            Err(e) => panic!("Couldn't connect to sway: {}", e)
        };

        if let Ok(workspaces) = block_in_place(|| {
            let wp = sway.ipc(IpcCommand::GetWorkspaces)?;
            let wp = String::from_utf8(wp).unwrap();
            let wp: Vec<Workspace> = serde_json::from_str(wp.as_str()).unwrap();
            Ok::<Vec<Workspace>, ksway::Error>(wp)
        }) {
            for wp in workspaces {
                let label = wp.as_label();
                content.add(&label);
                wp_map.insert(wp.num, label);
            }
        }

        let srx = sway.subscribe(vec![IpcEvent::Workspace]).unwrap();
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        spawn_blocking(move || {
            loop {
                while let Ok((_, payload)) = srx.try_recv() {
                    // payload_type is always going to be workspace since it's the only event we subscribed to,
                    // but if we subscribe to something else in the future please also check payload_type
                    let payload = String::from_utf8(payload).unwrap();
                    // println!("{}", payload);
                    let payload: WorkspaceEvent = serde_json::from_str(payload.as_str()).unwrap();
                    tx.send(payload).expect("Couldn't send workspace payload");
                }
                sway.poll().unwrap() // unwrap explicitly -> panic if polling fails
            }
        });

        {
            let content = content.clone();
            rx.attach(None, move |event| {
                match event.change.as_ref() {
                    "focus" => {
                        let old = event.old.unwrap();
                        let old_label = wp_map.get(&old.num).unwrap();
                        old_label.set_text(old.name.as_str());
                        old_label.get_style_context().remove_class("focused");

                        let current = event.current.unwrap();
                        let current_label = wp_map.get(&current.num).unwrap();
                        current_label.get_style_context().add_class("focused");

                        if current_label.get_parent().is_none() {
                            content.add(current_label);
                        }
                    }
                    "init" => {
                        let wp = event.current.unwrap();
                        let label = wp.as_label();
                        content.pack_start(
                            &label,
                            false,
                            true,
                            0
                        );
                        label.show();
                        wp_map.insert(wp.num, label);
                    }
                    "empty" => {
                        let label = wp_map.get(&event.current.unwrap().num).unwrap();
                        content.remove(label);
                    }
                    _ => ()
                }

                Continue(true)
            });
        }

        content
    }
}