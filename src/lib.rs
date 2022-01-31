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

impl Item {
    pub fn value(&self) -> Option<&str> {
        match self {
            Self::Single(v) | Self::Named(_, v) => Some(v),
            Self::None => None,
        }
    }
    pub fn value_mut(&mut self) -> Option<&mut str> {
        match self {
            Self::Single(v) | Self::Named(_, v) => Some(v),
            Self::None => None,
        }
    }
}

impl Data {
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }
    pub fn is_scalar(&self) -> bool {
        matches!(self, Self::Scalar(_))
    }
}

impl Wiki {
    pub fn new(kind: String, data: HashMap<String, Data>) -> Self {
        Self { kind, data }
    }
    pub fn get(&self, field: &str) -> Option<&Data> {
        self.data.get(field)
    }
    pub fn get_mut(&mut self, field: &str) -> Option<&mut Data> {
        self.data.get_mut(field)
    }
}