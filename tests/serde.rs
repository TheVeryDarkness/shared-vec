//! Tests for serde implementations
#![cfg(feature = "serde")]

use shared_vec::{String, Vec};
use std::cell::Cell;

#[test]
fn vec() {
    let original: Vec<Cell<usize>, u32> = Vec::from_boxed_slice(Box::new([1, 2, 3, 4, 5]));
    let serialized = serde_json::to_string(&original).unwrap();
    assert_eq!(serialized, "[1,2,3,4,5]");
    let deserialized: Vec<Cell<usize>, u32> = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original.as_slice(), deserialized.as_slice());
}

#[test]
fn string() {
    let original: String<Cell<usize>> = String::from_boxed_str("hello 🦀!".into());
    let serialized = serde_json::to_string(&original).unwrap();
    assert_eq!(serialized, "\"hello 🦀!\"");
    let deserialized: String<Cell<usize>> = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original.as_str(), deserialized.as_str());
}
