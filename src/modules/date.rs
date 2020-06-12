use chrono::Local;
use glib::Continue;
use gtk::{Label, LabelExt};
use serde::Deserialize;
use tokio::{
    spawn,
    time::delay_for,
};

use crate::modules::module::Module;

#[derive(Deserialize)]
pub struct DateModule {
    #[serde(default = "default_format")]
    format: String
}

fn default_format() -> String {
    String::from("%a %d %b %H:%M")
}

impl Module<Label> for DateModule {
    fn into_widget(self) -> Label {
        let date = gtk::Label::new(None);

        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        spawn(async move {
            let format = self.format.as_str();
            loop {
                let date = Local::now();
                let _ = tx.send(format!("{}", date.format(format)));
                delay_for(tokio::time::Duration::from_secs(1)).await;
            }
        });

        {
            let date = date.clone();
            rx.attach(None, move |s| {
                date.set_text(s.as_str());
                Continue(true)
            });
        }

        date
    }
}