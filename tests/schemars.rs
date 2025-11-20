//! Tests for schemars implementations
#![cfg(feature = "schemars")]

use schemars::{json_schema, schema_for};
use shared_vec::{String, Vec};
use std::cell::Cell;

#[test]
fn vec() {
    let s = schema_for!(Vec::<Cell<usize>, u32>);
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
}

#[test]
fn string() {
    let s = schema_for!(String::<Cell<usize>>);
    assert_eq!(
        s,
        json_schema!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "string",
            "title": "String",
        })
    );
}
