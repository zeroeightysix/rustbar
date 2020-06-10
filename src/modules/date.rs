use chrono::Local;
use gtk::{Label, LabelExt};

use async_trait::async_trait;

use crate::modules::module::Module;
use tokio::time::delay_for;

pub struct DateModule {}

#[async_trait]
impl Module<Label> for DateModule {
    fn new() -> Self {
        DateModule {}
    }

    async fn into_widget_handler(self) -> (Box<dyn FnMut()>, Label) {
        let date_label = gtk::Label::new(None);

        let (mut tx, mut rx) = tokio::sync::mpsc::channel(2);
        tokio::spawn(async move {
            loop {
                let date = Local::now();
                let _ = tx.send(format!("{}", date.format("%H:%M:%S"))).await;
                delay_for(tokio::time::Duration::from_secs(1)).await;
            }
        });

        let label = date_label.clone();
        let idle_func = move || {
            if let Ok(s) = rx.try_recv() {
                label.set_text(s.as_str());
            }
        };

        (Box::new(idle_func), date_label)
    }
}