//! Tests for error implementations.

use shared_vec::{ArcString, RcString};
use std::error::Error;

#[test]
fn format() {
    let s = "An error occurred";
    let debug = "\"An error occurred\"";
    let err: Box<dyn Error> = RcString::from(s).into();
    assert_eq!(format!("{}", err), s);
    assert_eq!(format!("{:?}", err), debug);

    let err: Box<dyn Error + Send> = ArcString::from(s).into();
    assert_eq!(format!("{}", err), s);
    assert_eq!(format!("{:?}", err), debug);

    let err: Box<dyn Error + Send + Sync> = ArcString::from(s).into();
    assert_eq!(format!("{}", err), s);
    assert_eq!(format!("{:?}", err), debug);
}
