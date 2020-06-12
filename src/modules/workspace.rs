use glib::Continue;
use gtk::{Label, LabelExt};
use ksway::{IpcCommand, IpcEvent};
use serde::Deserialize;
use tokio::task::{block_in_place, spawn_blocking};

use crate::modules::module::Module;

#[derive(Deserialize)]
pub struct WorkspaceModule {}

#[derive(Deserialize)]
struct WorkspaceEvent {
    change: String,
    // old: Option<IPCWorkspace>,
    current: Option<Workspace>,
}

#[derive(Deserialize, Debug)]
struct Workspace {
    name: String,
    focused: bool,
}

impl Module<gtk::Label> for WorkspaceModule {
    fn into_widget(self) -> Label {
        let content = Label::new(None);

        let mut sway = match ksway::client::Client::connect() {
            Ok(client) => client,
            Err(e) => panic!("Couldn't connect to sway: {}", e)
        };

        let _ = block_in_place(|| {
            let wp = sway.ipc(IpcCommand::GetWorkspaces)?;
            let wp = String::from_utf8(wp).unwrap();
            let wp: Vec<Workspace> = serde_json::from_str(wp.as_str()).unwrap();
            Ok::<Vec<Workspace>, ksway::Error>(wp)
        });

        let srx = sway.subscribe(vec![IpcEvent::Workspace]).unwrap();
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        spawn_blocking(move || {
            loop {
                while let Ok((_, payload)) = srx.try_recv() {
                    // payload_type is always going to be workspace since it's the only event we subscribed to,
                    // but if we subscribe to something else in the future please also check payload_type
                    let payload = String::from_utf8(payload).unwrap();
                    let payload: WorkspaceEvent = serde_json::from_str(payload.as_str()).unwrap();
                    if payload.change == "focus" {
                        let _ = tx.send(payload.current.unwrap().name);
                    }
                }
                sway.poll().unwrap() // unwrap explicitly -> panic if polling fails
            }
        });

        {
            let content = content.clone();
            rx.attach(None, move |name| {
                content.set_text(name.as_str());
                Continue(true)
            });
        }

        content
    }
}