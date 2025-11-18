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
    assert_eq!(String::<C>::new().len(), 0);
    assert_eq!(String::<C>::new().as_str(), "");
    assert_eq!(String::<C>::new(), String::<C>::default());
    assert_eq!(String::<C>::new(), String::<C>::from_str("".into()));
    assert_eq!(
        String::<C>::new(),
        String::<C>::from_utf8(b"".to_vec().into_boxed_slice()).unwrap()
    );

    assert_eq!(
        String::<C>::from_str("hello 🦀!".into()),
        String::<C>::from_utf8("hello 🦀!".as_bytes().to_vec().into_boxed_slice()).unwrap()
    );
    assert_eq!(String::<C>::from_str("hello 🦀!".into()), unsafe {
        String::<C>::from_utf8_unchecked("hello 🦀!".as_bytes().to_vec().into_boxed_slice())
    });

    let s = String::<C>::from_str("hello 🦀!".to_owned().into_boxed_str());
    assert_eq!(s.len(), 11);
    assert_eq!(s.as_str(), "hello 🦀!");

    let s2 = s.clone();
    assert_eq!(s2.len(), 11);
    assert_eq!(s2.as_str(), "hello 🦀!");

    macro_rules! test_idx {
        ($bounds:expr, $string:literal) => {
            let s3 = s.idx($bounds);
            assert_eq!(s3.len(), $string.len());
            assert_eq!(s3.as_str(), $string);
        };
    }

    test_idx!(..5, "hello");
    test_idx!(..11, "hello 🦀!");
    test_idx!(..=10, "hello 🦀!");
    test_idx!(..=9, "hello 🦀");
    test_idx!(.., "hello 🦀!");
    test_idx!(6..10, "🦀");
    test_idx!(6.., "🦀!");
    test_idx!(6..=10, "🦀!");
}

#[test]
fn rc_string() {
    test_string::<Cell<usize>>();
}

#[test]
fn arc_string() {
    test_string::<AtomicUsize>();
}
