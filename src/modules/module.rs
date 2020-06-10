use glib::IsA;
use gtk::Widget;
use async_trait::async_trait;

#[async_trait]
pub trait Module<W>
    where W: IsA<Widget> {
    async fn create_module() -> (Box<dyn FnMut()>, W);
}