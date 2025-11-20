//! Tests for serde implementations
#![cfg(feature = "serde")]

use shared_vec::{RcString, RcVec};

#[test]
fn vec() {
    let original: RcVec<u32> = RcVec::from_boxed_slice(Box::new([1, 2, 3, 4, 5]));
    let serialized = serde_json::to_string(&original).unwrap();
    assert_eq!(serialized, "[1,2,3,4,5]");
    let deserialized: RcVec<u32> = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original.as_slice(), deserialized.as_slice());
}

#[test]
fn string() {
    let original: RcString = RcString::from_boxed_str("hello 🦀!".into());
    let serialized = serde_json::to_string(&original).unwrap();
    assert_eq!(serialized, "\"hello 🦀!\"");
    let deserialized: RcString = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original.as_str(), deserialized.as_str());
}

#[test]
fn vec_string() {
    let original: RcVec<RcString> = RcVec::from_boxed_slice(Box::new(
        ["hello", "world"].map(|s| RcString::from_boxed_str(s.into())),
    ));
    let serialized = serde_json::to_string(&original).unwrap();
    assert_eq!(serialized, "[\"hello\",\"world\"]");
    let deserialized: RcVec<RcString> = serde_json::from_str(&serialized).unwrap();
    assert_eq!(original.as_slice(), deserialized.as_slice());
}
