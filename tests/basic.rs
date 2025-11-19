use shared_vec::{Counter, String, Vec};
use std::borrow::ToOwned;
use std::cell::Cell;
use std::collections::HashSet;
use std::hash::RandomState;
use std::ops::Bound;
use std::sync::atomic::AtomicUsize;

#[expect(clippy::reversed_empty_ranges)]
fn test_vec<C: Counter<usize>>() {
    assert_eq!(Vec::<C, u8>::new().len(), 0);
    assert_eq!(Vec::<C, u8>::default().len(), 0);

    let vec = [1, 2, 3, 4, 5];

    let v = Vec::<C, _>::from_boxed_slice(Box::new([1, 2, 3, 4, 5]));
    assert_eq!(v.len(), 5);
    assert_eq!(&*v, &[1, 2, 3, 4, 5]);
    assert_eq!(v, v.idx(0..5));
    assert_eq!(v, Vec::<C, _>::from(Box::new([1, 2, 3, 4, 5]) as Box<[_]>));

    assert_eq!(format!("{v:?}"), "[1, 2, 3, 4, 5]");

    let v2 = v.clone();
    assert_eq!(v2.len(), 5);
    assert_eq!(&*v2, &[1, 2, 3, 4, 5]);

    let v3 = v.get(1..4).unwrap();
    assert_eq!(v3.len(), 3);
    assert_eq!(&*v3, &[2, 3, 4]);

    let v4 = v.idx(0..2);
    assert_eq!(v4.len(), 2);
    assert_eq!(&*v4, &[1, 2]);

    for i in 0..v.len() {
        assert_eq!(v.idx(i..i + 1).len(), 1);
        assert_eq!(v.idx(i..i + 1).as_slice(), &[v.as_slice()[i]]);
    }

    let mut s = v.clone();
    for i in 0..v.len() {
        assert_eq!(s, v.idx(i..));
        s = s.idx(1..);
    }

    let mut s = v.clone();
    for i in (0..v.len()).rev() {
        assert_eq!(s, v.idx(..(i + 1)));
        s = s.idx(..i);
    }

    macro_rules! idx_ok {
        ($bounds:expr, $slice:expr) => {
            let bounds = $bounds;
            let slice = $slice;
            let v3 = v.get(bounds.clone()).unwrap();
            assert_eq!(v3.len(), slice.len());
            assert_eq!(&*v3, slice);
            assert_eq!(v3.as_slice(), &vec[bounds.clone()]);
            assert_eq!(<_ as AsRef<[_]>>::as_ref(&v3), &vec[bounds.clone()]);

            assert!(v.is_valid_range(bounds.clone()));
            assert_eq!(v3, unsafe { v.get_unchecked(bounds) });
        };
    }

    idx_ok!(0..0, &[]);
    idx_ok!(2..2, &[]);
    idx_ok!(5..5, &[]);
    idx_ok!((Bound::Excluded(1), Bound::Included(1)), &[]);
    idx_ok!(..3, &[1, 2, 3]);
    idx_ok!(2.., &[3, 4, 5]);
    idx_ok!(1..4, &[2, 3, 4]);
    idx_ok!(1..=3, &[2, 3, 4]);

    macro_rules! idx_err {
        ($bounds:expr) => {
            assert!(v.get($bounds).is_none());
            assert!(vec.get($bounds).is_none());
        };
    }

    idx_err!(4..2);
    idx_err!(..6);
    idx_err!(6..);
    idx_err!((usize::MAX)..);
    idx_err!(..(usize::MAX));
    idx_err!(..=(usize::MAX));
    idx_err!((Bound::Excluded(usize::MAX), Bound::Unbounded));

    let integers: [&[i32]; 3] = [&[0, 1], &[3], &[6, 7, 8]];
    assert!(integers.is_sorted());

    let integers = integers
        .iter()
        .map(|s| Vec::<C, i32>::from_boxed_slice(s.to_vec().into_boxed_slice()))
        .collect::<std::vec::Vec<Vec<C, i32>>>();
    for s in integers.iter() {
        assert!(integers.binary_search(s).is_ok());
    }
    assert!(integers.is_sorted());

    let integers = Vec::<C, i32>::from_boxed_slice(Box::new([5, 3, 1, 4, 2]));
    let slices = [
        integers.idx(2..=2),
        integers.idx(2..),
        integers.idx(4..),
        integers.idx(1..),
        integers.idx(0..2),
        integers.idx(0..),
    ];
    assert!(slices.is_sorted());
}

#[test]
fn rc_vec() {
    test_vec::<Cell<usize>>();
}

#[test]
fn arc_vec() {
    test_vec::<AtomicUsize>();
}

#[expect(clippy::reversed_empty_ranges)]
fn test_string<C: Counter<usize>>() {
    assert_eq!(String::<C>::new().len(), 0);
    assert_eq!(String::<C>::new().as_str(), "");
    assert_eq!(String::<C>::new(), String::<C>::default());
    assert_eq!(String::<C>::new(), String::<C>::from_boxed_str("".into()));
    assert_eq!(
        String::<C>::new(),
        String::<C>::from_utf8(b"".to_vec().into_boxed_slice()).unwrap()
    );

    let string = "hello 🦀!".to_owned();

    assert_eq!(
        String::<C>::from_boxed_str("hello 🦀!".into()),
        String::<C>::from_utf8("hello 🦀!".as_bytes().to_vec().into_boxed_slice()).unwrap()
    );
    assert_eq!(String::<C>::from_boxed_str("hello 🦀!".into()), unsafe {
        String::<C>::from_utf8_unchecked("hello 🦀!".as_bytes().to_vec().into_boxed_slice())
    });
    assert_eq!(String::<C>::from_boxed_str("hello 🦀!".into()), unsafe {
        String::<C>::from_utf8_unchecked("hello 🦀!".as_bytes().to_vec().into())
    });
    assert_eq!(
        String::<C>::from_boxed_str("hello 🦀!".into()),
        String::<C>::from("hello 🦀!".to_owned().into_boxed_str())
    );

    assert!(String::<C>::from_utf8(b"\xFF".to_vec().into_boxed_slice()).is_err());

    let s = String::<C>::from_boxed_str("hello 🦀!".to_owned().into_boxed_str());
    assert_eq!(s.len(), 11);
    assert_eq!(s.as_str(), "hello 🦀!");

    let s2 = s.clone();
    assert_eq!(s2.len(), 11);
    assert_eq!(s2.as_str(), "hello 🦀!");

    macro_rules! idx_ok {
        ($bounds:expr, $string:literal) => {
            let bounds = $bounds;
            let s3 = s.idx(bounds.clone());
            assert_eq!(s3.as_str(), &string[bounds.clone()]);
            assert_eq!(<_ as AsRef<str>>::as_ref(&s3), &string[bounds.clone()]);
            assert_eq!(<_ as AsRef<[u8]>>::as_ref(&s3), string[bounds].as_bytes());
            assert_eq!(s3.len(), $string.len());
            assert_eq!(s3.is_empty(), $string.is_empty());
            assert_eq!(s3.as_str(), $string);
            assert_eq!(s3.to_string(), $string);
            assert_eq!(s3.bytes(), $string.as_bytes());
            assert_eq!(format!("{s3:?}"), format!("{:?}", $string));

            assert!(s.is_valid_range($bounds));

            let s4 = unsafe { s.get_unchecked($bounds) };
            assert_eq!(s4.len(), $string.len());
            assert_eq!(s4.is_empty(), $string.is_empty());
            assert_eq!(s4.as_str(), $string);
            assert_eq!(s4.to_string(), $string);
            assert_eq!(s4.bytes(), $string.as_bytes());
            assert_eq!(format!("{s4:?}"), format!("{:?}", $string));

            assert_eq!(s3, s4);
        };
    }

    idx_ok!(0..0, "");
    idx_ok!(1..1, "");
    idx_ok!(11..11, "");
    idx_ok!((Bound::Excluded(1), Bound::Excluded(2)), "");
    idx_ok!(..5, "hello");
    idx_ok!(..11, "hello 🦀!");
    idx_ok!(..=10, "hello 🦀!");
    idx_ok!(..=9, "hello 🦀");
    idx_ok!(.., "hello 🦀!");
    idx_ok!(6..10, "🦀");
    idx_ok!(6.., "🦀!");
    idx_ok!(6..=10, "🦀!");

    macro_rules! idx_err {
        ($bounds:expr) => {
            let s3 = s.get($bounds);
            assert!(s3.is_none());
        };
    }

    idx_err!(4..2);
    idx_err!(6..9);
    idx_err!(..12);
    idx_err!(12..);
    idx_err!((usize::MAX)..);
    idx_err!(..(usize::MAX));
    idx_err!(..=(usize::MAX));
    idx_err!((Bound::Excluded(usize::MAX), Bound::Unbounded));

    let strings = [
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
    ];
    let strings = strings
        .iter()
        .map(|s| String::<C>::from_boxed_str((*s).to_owned().into_boxed_str()))
        .collect::<std::vec::Vec<String<C>>>();
    for s in strings.iter() {
        assert!(strings.binary_search(s).is_ok());
    }
    assert!(strings.is_sorted());
    let set: HashSet<String<C>, RandomState> = HashSet::from_iter(strings.iter().cloned());
    assert_eq!(set.len(), strings.len());
}

#[test]
fn rc_string() {
    test_string::<Cell<usize>>();
}

#[test]
fn arc_string() {
    test_string::<AtomicUsize>();
}
