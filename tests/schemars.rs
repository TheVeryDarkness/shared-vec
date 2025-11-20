//! Tests for schemars implementations
#![cfg(feature = "schemars")]

use schemars::{json_schema, schema_for, JsonSchema};
use shared_vec::{RcString, RcVec};

#[test]
fn vec() {
    assert_eq!(
        RcVec::<u32>::inline_schema(),
        <[u32] as schemars::JsonSchema>::inline_schema()
    );
    let s = schema_for!(RcVec<u32>);
    assert_eq!(
        s,
        json_schema!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "title": "Vec_of_uint32",
            "items": {
                "type": "integer",
                "format": "uint32",
                "minimum": 0,
            }
        })
    );
    let s_ = schema_for!(std::vec::Vec::<u32>);
    assert_eq!(s.as_value()["type"], s_.as_value()["type"]);
    assert_eq!(s.as_value()["items"], s_.as_value()["items"]);
}

#[test]
fn string() {
    assert_eq!(
        RcString::inline_schema(),
        <str as schemars::JsonSchema>::inline_schema()
    );
    assert_eq!(
        RcString::inline_schema(),
        <std::string::String as schemars::JsonSchema>::inline_schema()
    );
    let s = schema_for!(RcString);
    assert_eq!(
        s,
        json_schema!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "string",
            "title": "String",
        })
    );
    let s_ = schema_for!(std::string::String);
    assert_eq!(s.as_value()["type"], s_.as_value()["type"]);
    assert_eq!(s.as_value()["items"], s_.as_value()["items"]);
}
