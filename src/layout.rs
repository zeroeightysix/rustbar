use std::collections::HashMap;

use gtk::{
    Orientation,
    prelude::*,
};
use serde::{Deserialize, Serialize};
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

#[derive(Deserialize, Serialize, Eq, PartialEq, Hash, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Position {
    Left,
    Centre,
    Right,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Group {
    Positions(Box<HashMap<Position, Group>>),
    // We never really keep the module struct around, so let's not try to do it here either
    Modules(Vec<Value>),
}

impl Default for Group {
    fn default() -> Self {
        Modules(Vec::default())
    }
}

impl Group {
    pub fn initialise_handlers(&self, content: &gtk::Box, idle_functions: &mut Vec<Box<dyn FnMut()>>) {
        match self {
            Modules(modules) => {
                for m in modules {
                    if let Some(name) = m["name"].as_str() {
                        // We use a macro here because the module is of varying type.
                        add_module!(name, content, idle_functions, m,
                            "date" => DateModule;
                            "hello" => HelloModule;
                            "workspaces" => WorkspaceModule
                        );
                    }
                };
            }
            Positions(map) => {
                for (p, g) in map.iter() {
                    let new = gtk::Box::new(Orientation::Horizontal, 0);
                    match p {
                        Position::Left => content.add(&new),
                        Position::Centre => content.set_center_widget(Some(&new)),
                        Position::Right => content.pack_end(&new, false, false, 0)
                    };
                    g.initialise_handlers(&new, idle_functions)
                }
            }
        }
    }
}