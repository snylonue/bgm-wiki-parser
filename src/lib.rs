use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Item {
    Single(String),
    Named(String, String),
    None,
}

pub enum Data {
    Array(Vec<Item>),
    Scalar(String),
}

pub struct Wiki {
    kind: String,
    data: HashMap<String, Data>,
}

