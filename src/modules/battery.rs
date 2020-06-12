use futures::executor::block_on;
use glib::{
    bitflags::_core::time::Duration,
    Continue,
};
use gtk::{Label, LabelExt};
use serde::Deserialize;
use tokio::{
    task::spawn_blocking,
    time::delay_for,
};

use crate::modules::module::Module;

#[derive(Deserialize)]
pub struct BatteryModule {}

impl Module<Label> for BatteryModule {
    fn into_widget(self) -> Label {
        let label = Label::new(None);

        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        spawn_blocking(move || {
            let manager = battery::Manager::new().unwrap();
            let mut battery = match manager.batteries().unwrap().nth(1) {
                Some(Ok(b)) => b,
                Some(Err(why)) => panic!("Couldn't find any batteries: {}", why),
                None => panic!("Couldn't access batteries")
            };

            loop {
                let _ = tx.send(format!("{:?}", battery.energy() / battery.energy_full()));
                block_on(delay_for(Duration::from_secs(1)));
                let _ = manager.refresh(&mut battery);
            }
        });

        {
            let label = label.clone();
            rx.attach(None, move |text| {
                label.set_text(text.as_str());
                Continue(true)
            });
        }

        label
    }
}