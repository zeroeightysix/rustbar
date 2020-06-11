use gtk::Button;
use serde::Deserialize;

use crate::modules::module::Module;

#[derive(Deserialize)]
pub struct HelloModule {}

impl Module<Button> for HelloModule {
    fn into_widget_handler(self) -> (Box<dyn FnMut()>, Button) {
        let button = gtk::Button::new();

        (Box::new(|| {}), button)
    }
}