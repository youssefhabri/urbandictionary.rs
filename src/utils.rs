use serde_json::Value;
use std::collections::BTreeMap;
use ::error::{Error, Result};

#[macro_escape]
macro_rules! req {
    ($opt:expr) => {
        try!($opt.ok_or(Error::Decode(concat!("Type mismatch in model:", line!(), ": ", stringify!($opt)), Value::Null)))
    }
}

pub fn into_map(value: Value) -> Result<BTreeMap<String, Value>> {
    match value {
        Value::Object(m) => Ok(m),
        value => Err(Error::Decode("Expected object", value)),
    }
}

pub fn decode_array<T, F: Fn(Value) -> Result<T>>(value: Value, f: F) -> Result<Vec<T>> {
    into_array(value).and_then(|x| x.into_iter().map(f).collect())
}

pub fn into_array(value: Value) -> Result<Vec<Value>> {
    match value {
        Value::Array(v) => Ok(v),
        value => Err(Error::Decode("Expected array", value)),
    }
}

pub fn into_string(value: Value) -> Result<String> {
    match value {
        Value::String(s) => Ok(s),
        value => Err(Error::Decode("Expected string", value)),
    }
}

pub fn remove(map: &mut BTreeMap<String, Value>, key: &str) -> Result<Value> {
    map.remove(key).ok_or(Error::Decode("Unexpected absent key", Value::String(key.into())))
}
