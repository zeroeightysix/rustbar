use glib::IsA;
use gtk::Widget;
use async_trait::async_trait;
use serde_json::Value;
use serde::de::DeserializeOwned;

#[async_trait]
pub trait Module<W>
    where W: IsA<Widget> {
    async fn into_widget_handler(self) -> (Box<dyn FnMut()>, W);

    fn from_value(v: &Value) -> Box<Self>
        where Self: DeserializeOwned {
        serde_json::from_value(v.clone()).unwrap()
    }
}