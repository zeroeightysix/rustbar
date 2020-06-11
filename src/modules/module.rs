use glib::IsA;
use gtk::Widget;
use serde::de::DeserializeOwned;
use serde_json::Value;

pub trait Module<W>
    where W: IsA<Widget> {
    fn into_widget_handler(self) -> (Box<dyn FnMut()>, W);

    fn from_value(v: &Value) -> Box<Self>
        where Self: DeserializeOwned {
        serde_json::from_value(v.clone()).unwrap()
    }
}