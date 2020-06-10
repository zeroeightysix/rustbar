use gtk::Button;

use serde::Deserialize;
use async_trait::async_trait;

use crate::modules::module::Module;

#[derive(Deserialize)]
pub struct HelloModule {}

#[async_trait]
impl Module<Button> for HelloModule {
    async fn into_widget_handler(self) -> (Box<dyn FnMut()>, Button) {
        let button = gtk::Button::new();

        (Box::new(|| {}), button)
    }
}