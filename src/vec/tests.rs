use crate::{Counter, String, Vec};
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use core::cell::Cell;
use core::sync::atomic::AtomicUsize;

fn test_vec<C: Counter<usize>>() {
    let v = Vec::<C, _>::from_boxed_slice(Box::new([1, 2, 3, 4, 5]));
    assert_eq!(v.len(), 5);

    let v2 = v.clone();
    assert_eq!(v2.len(), 5);
    assert_eq!(&*v2, &[1, 2, 3, 4, 5]);

    let v3 = v.get(1..4).unwrap();
    assert_eq!(v3.len(), 3);
    assert_eq!(&*v3, &[2, 3, 4]);

    let v4 = v.idx(0..2);
    assert_eq!(v4.len(), 2);
    assert_eq!(&*v4, &[1, 2]);
}

#[test]
fn rc_vec() {
    test_vec::<Cell<usize>>();
}

#[test]
fn arc_vec() {
    test_vec::<AtomicUsize>();
}

fn test_string<C: Counter<usize>>() {
    let s = String::<C>::from_str("hello 🦀!".to_owned().into_boxed_str());
    assert_eq!(s.len(), 11);
    assert_eq!(s.as_str(), "hello 🦀!");

    let s2 = s.clone();
    assert_eq!(s2.len(), 11);
    assert_eq!(s2.as_str(), "hello 🦀!");

    let s3 = s.get(6..10).unwrap();
    assert_eq!(s3.len(), 4);
    assert_eq!(s3.as_str(), "🦀");

    let s4 = s.idx(0..5);
    assert_eq!(s4.len(), 5);
    assert_eq!(s4.as_str(), "hello");
}

#[test]
fn rc_string() {
    test_string::<Cell<usize>>();
}

#[test]
fn arc_string() {
    test_string::<AtomicUsize>();
}
