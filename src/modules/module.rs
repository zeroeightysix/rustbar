extern crate gtk;

use gtk::WidgetExt;

pub struct Module<'a, T, M> where T: WidgetExt {
    widget: T,
    message_handler: &'a Fn(&Self, &M),
}

impl<'a, 'b, T: WidgetExt, M> Module<'a, T, M> {
    pub fn get_widget(&self) -> &T {
        &self.widget
    }

    pub fn handle(&self, message: &'b M) {
        (self.message_handler)(self, message);
    }

    pub fn new(widget: T, handler: &'a Fn(&Module<'a, T, M>, &M)) -> Module<'a, T, M> {
        Module {
            widget: widget,
            message_handler: handler
        }
    }
}