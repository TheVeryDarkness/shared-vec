#![cfg(feature = "schemars")]

use crate::{Counter, String, Vec};
use alloc::{borrow::Cow, format};
use schemars::{JsonSchema, SchemaGenerator};

impl<C: Counter<usize>, T: JsonSchema> JsonSchema for Vec<C, T> {
    fn inline_schema() -> bool {
        <[T] as JsonSchema>::inline_schema()
    }

    fn schema_name() -> Cow<'static, str> {
        format!("Vec_of_{}", T::schema_name()).into()
    }

    fn schema_id() -> Cow<'static, str> {
        format!("Vec<{}>", T::schema_id()).into()
    }

    #[inline]
    fn json_schema(_generator: &mut SchemaGenerator) -> schemars::Schema {
        <[T] as JsonSchema>::json_schema(_generator)
    }
}

impl<C: Counter<usize>> JsonSchema for String<C> {
    fn inline_schema() -> bool {
        <str as JsonSchema>::inline_schema()
    }

    fn schema_name() -> Cow<'static, str> {
        "String".into()
    }

    fn schema_id() -> Cow<'static, str> {
        "String".into()
    }

    #[inline]
    fn json_schema(_generator: &mut SchemaGenerator) -> schemars::Schema {
        <str as JsonSchema>::json_schema(_generator)
    }
}
