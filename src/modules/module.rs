use glib::IsA;
use gtk::Widget;
use async_trait::async_trait;

#[async_trait]
pub trait Module<W>
    where W: IsA<Widget> {
    fn new() -> Self;
    async fn into_widget_handler(self) -> (Box<dyn FnMut()>, W);
}