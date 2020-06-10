use serde::{
    Deserialize,
    Serialize
};

#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub anchors: Anchors,
    #[serde(default)]
    pub margins: Margins,
    #[serde(default)]
    pub modules: Vec<serde_json::Value>
}

#[derive(Deserialize, Serialize)]
pub struct Anchors {
    #[serde(default = "default_true")]
    pub top: bool,
    #[serde(default = "default_false")]
    pub bottom: bool,
    #[serde(default = "default_true")]
    pub left: bool,
    #[serde(default = "default_true")]
    pub right: bool,
}

impl Default for Anchors {
    fn default() -> Self {
        Anchors {
            top: true,
            bottom: false,
            left: true,
            right: true
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Margins {
    #[serde(default = "default_zero")]
    pub top: i32,
    #[serde(default = "default_zero")]
    pub bottom: i32,
    #[serde(default = "default_zero")]
    pub left: i32,
    #[serde(default = "default_zero")]
    pub right: i32,
}

impl Default for Margins {
    fn default() -> Self {
        Margins {
            top: 0,
            bottom: 0,
            left: 0,
            right: 0
        }
    }
}

// I don't get why serde doesn't support literal defaults yet. Am I missing something?
fn default_true() -> bool { true }
fn default_false() -> bool { false }
fn default_zero() -> i32 { 0 }