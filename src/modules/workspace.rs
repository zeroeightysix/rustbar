use serde::Deserialize;

use async_trait::async_trait;

use crate::modules::module::Module;
use ksway::IpcEvent;
use gtk::LabelExt;

#[derive(Deserialize)]
pub struct WorkspaceModule {}

#[derive(Deserialize)]
struct IPCWorkspacePayload {
    change: String,
    // old: Option<IPCWorkspace>,
    current: Option<IPCWorkspace>,
}

#[derive(Deserialize)]
struct IPCWorkspace {
    name: String
}

#[async_trait]
impl Module<gtk::Label> for WorkspaceModule {
    async fn into_widget_handler(self) -> (Box<dyn FnMut()>, gtk::Label) {
        let content = gtk::Label::new(None);

        let mut sway = match ksway::client::Client::connect() {
            Ok(client) => client,
            Err(e) => panic!("Couldn't connect to sway: {}", e)
        };

        let srx = sway.subscribe(vec![IpcEvent::Workspace]).unwrap();
        let (mut tx, mut rx) = tokio::sync::mpsc::channel(10);
        tokio::spawn(async move {
            loop {
                while let Ok((_, payload)) = srx.try_recv() {
                    // payload_type is always going to be workspace since it's the only event we subscribed to,
                    // but if we subscribe to something else in the future please also check payload_type
                    let payload = String::from_utf8(payload).unwrap();
                    let payload: IPCWorkspacePayload = serde_json::from_str(payload.as_str()).unwrap();
                    if payload.change == "focus" {
                        let _ = tx.send(payload.current.unwrap().name).await;
                    }
                }
                sway.poll().unwrap(); // explicitly panic if an error occurs while polling
            }
        });

        let content_clone = content.clone();
        (box move || {
            if let Ok(name) = rx.try_recv() {
                content.set_text(name.as_str());
            }
        }, content_clone)
    }
}