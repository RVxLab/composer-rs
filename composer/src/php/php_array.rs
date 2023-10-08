use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An enum that represents an array in PHP
/// Due to the way PHP arrays are typed, it can result in an array or object in the JSON
///
/// Examples:
/// ```php
/// $emptyArray = []; // []"
/// $indexedArray = [1, 2, 3, 4]; // [1, 2, 3, 4]
/// $associativeArray = ['name' => 'Alice', 'age' => 31]; // {"name": "Alice", "age": 31}
/// ```
///
/// Therefore it's not 100% predictable if a JSON key expecting to be have map is actually a map when empty
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PhpArray<TItem> {
    Indexed(Vec<TItem>),
    Associative(HashMap<String, TItem>),
}
