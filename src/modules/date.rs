use chrono::Local;
use gtk::{Label, LabelExt};

use async_trait::async_trait;

use crate::modules::module::Module;
use tokio::time::delay_for;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DateModule {
    #[serde(default = "default_format")]
    format: String
}

fn default_format() -> String {
    String::from("%H:%M:%S")
}

#[async_trait]
impl Module<Label> for DateModule {
    async fn into_widget_handler(self) -> (Box<dyn FnMut()>, Label) {
        let date_label = gtk::Label::new(None);

        let (mut tx, mut rx) = tokio::sync::mpsc::channel(2);
        tokio::spawn(async move {
            let format = self.format.as_str();
            loop {
                let date = Local::now();
                let _ = tx.send(format!("{}", date.format(format))).await;
                delay_for(tokio::time::Duration::from_secs(1)).await;
            }
        });

        let label = date_label.clone();
        (Box::new(move || {
            if let Ok(s) = rx.try_recv() {
                label.set_text(s.as_str());
            }
        }), date_label)
    }
}