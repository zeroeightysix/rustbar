extern crate gtk;

use gtk::WidgetExt;

pub struct Module<'a, T, M> where T: WidgetExt {
    widget: T,
    message_handler: &'a Fn(&'a M),
}

impl<'a, T: WidgetExt, M> Module<'a, T, M> {
    pub fn get_widget(&self) -> &T {
        &self.widget
    }

    pub fn handle(&self, message: &'a M) {
        (self.message_handler)(message);
    }

    pub fn new(widget: T, handler: &'a Fn(&'a M)) -> Module<'a, T, M> {
        Module {
            widget: widget,
            message_handler: handler
        }
    }
}