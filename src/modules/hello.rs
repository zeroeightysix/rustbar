use gtk::Button;

use async_trait::async_trait;

use crate::modules::module::Module;

pub struct HelloModule {}

#[async_trait]
impl Module<Button> for HelloModule {
    fn new() -> Self {
        HelloModule {}
    }

    async fn into_widget_handler(self) -> (Box<dyn FnMut()>, Button) {
        let button = gtk::Button::new();

        (Box::new(|| {}), button)
    }
}