use std::collections::HashMap;

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum JSONtype {
    Bool(bool),
    String(String),
    Int(i64),
    Float(f64),
    List(Vec<JSONtype>),
    Object(HashMap<String, JSONtype>),
    Null,
}