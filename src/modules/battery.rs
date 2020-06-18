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
use crate::modules::battery::BatteryFormat::Percentage;

#[derive(Deserialize)]
pub struct BatteryModule {
    #[serde(default)]
    format: BatteryFormat
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "precision")]
enum BatteryFormat {
    Percentage(usize),
    Floating(usize)
}

impl Default for BatteryFormat {
    fn default() -> Self {
        Percentage(0)
    }
}

impl BatteryFormat {
    fn format(&self, input: f32) -> String {
        match self {
            BatteryFormat::Percentage(p) => {
                format!("{:.*}%", p, input * 100.)
            },
            BatteryFormat::Floating(p) => {
                format!("{:.*}", p, input)
            }
        }
    }
}

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

            let format = self.format;
            loop {
                let _ = tx.send(format.format((battery.energy() / battery.energy_full()).value));
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