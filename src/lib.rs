pub mod parser;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Item {
    Single(String),
    Named(String, String),
    None,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Data {
    Array(Vec<Item>),
    Scalar(String),
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub struct Wiki {
    #[serde(rename = "type")]
    pub kind: String,
    pub data: HashMap<String, Data>,
}
