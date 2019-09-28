extern crate gtk;

use gtk::WidgetExt;

pub struct Module<'a, T, M>
where
    T: WidgetExt {
    pub widget: T,
    pub message_handler: &'a Fn(&'a M),
}