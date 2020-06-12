use chrono::Local;
use gtk::{Label, LabelExt};
use serde::Deserialize;
use tokio::time::delay_for;

use async_trait::async_trait;

use crate::modules::module::Module;
use glib::Continue;

#[derive(Deserialize)]
pub struct DateModule {
    #[serde(default = "default_format")]
    format: String
}

fn default_format() -> String {
    String::from("%a %d %b %H:%M")
}

#[async_trait]
impl Module<Label> for DateModule {
    fn into_widget(self) -> Label {
        let date_label = gtk::Label::new(None);

        let (mut tx, mut rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        tokio::spawn(async move {
            let format = self.format.as_str();
            loop {
                let date = Local::now();
                let _ = tx.send(format!("{}", date.format(format)));
                delay_for(tokio::time::Duration::from_secs(1)).await;
            }
        });

        let cl = date_label.clone();

        rx.attach(None, move |s| {
            date_label.set_text(s.as_str());
            Continue(true)
        });

        cl
    }
}