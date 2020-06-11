use std::collections::HashMap;

use gtk::prelude::*;
use serde_json::Value;

use crate::{
    layout::Group::{Modules, Positions},
    modules::{
        date::DateModule,
        hello::HelloModule,
        module::Module,
        workspace::WorkspaceModule,
    },
};

macro_rules! add_module {
    (
        $nm:expr,
        $cb:expr,
        $fn:expr,
        $js:expr,
        $(
            $name:expr => $m:ident
        );*
    ) => {
        $(
            if $nm == $name {
                let (f, w) = $m::from_value($js).into_widget_handler();
                $fn.push(f);
                $cb.add(&w);
            }
        )*
    }
}

pub enum Group {
    Positions(Box<HashMap<u8, Group>>),
    // We never really keep the module struct around, so let's not try to do it here either
    Modules(Vec<Value>),
}

impl From<&Value> for Group {
    fn from(v: &Value) -> Self {
        if let Some(v) = v.as_array() {
            Modules(v.clone())
        } else if let Some(v) = v.as_object() {
            Positions(box v.into_iter().map(|(key, value)| {
                let key = match key.as_str() {
                    "left" => 0,
                    "center" => 50,
                    "centre" => 50,
                    "right" => 100,
                    f => match f.parse() {
                        Ok(f) => f,
                        Err(e) => panic!("Positions must be numerical! {}", e)
                    }
                };
                let value: Group = value.into();
                (key, value)
            }).collect())
        } else {
            panic!("Tried to create group from a Value that is not an array or object: {}", v)
        }
    }
}

impl Group {
    pub fn initialise_handlers(&self, content_box: &gtk::Box, idle_functions: &mut Vec<Box<dyn FnMut()>>) {
        match self {
            Modules(modules) => {
                for m in modules {
                    if let Some(name) = m["name"].as_str() {
                        // We use a macro here because the module is of varying type.
                        add_module!(name, content_box, idle_functions, m,
                            "date" => DateModule;
                            "hello" => HelloModule;
                            "workspaces" => WorkspaceModule
                        );
                    }
                }
            }
            Positions(map) => {
                for g in map.values() {
                    g.initialise_handlers(content_box, idle_functions)
                }
            }
        }
    }
}