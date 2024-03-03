use std::collections::HashMap;

pub enum Value {
    Number(i64),
    True,
    False,
    String(String),
    Array(Vec<Value>),
    Object(HashMap<(String, Value)>),
}

pub fn parse(input: &str) -> Result<Value, &'static str> {
    let remove_whitespace: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    let mut chars = remove_whitespace.chars();
}

// match char is not or correct
// pub fn match_char()
