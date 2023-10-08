use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PhpArray {
    Indexed(Vec<Value>),
    Associative(HashMap<String, Value>),
}
