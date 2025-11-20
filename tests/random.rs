//! Randomized tests for [`shared_vec`] crate

use itertools::Itertools;
use shared_vec::{ArcString, Counter, RcString, String};
use std::borrow::{Borrow, Cow};
use std::cell::Cell;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicUsize;

fn test_string<C: Counter<usize>>() {
    let mut strings = [
        'a', 'b', '¥', '±', 'α', 'β', '₍', '₎', '₤', '€', '⻰', '⻳', '🦀', '🧐', '𝆒', '𝆓',
    ]
    .iter()
    .copied()
    .permutations(3)
    .map(|s| String::<C>::from(std::string::String::from_iter(s)))
    .collect::<Vec<String<C>>>();

    strings.sort_by(|a, b| a.as_str().cmp(b.as_str()));
    assert!(strings.iter().all(|w| w.chars().count() == 3));
    assert!(strings.is_sorted());

    for s in strings.iter() {
        assert_eq!(PathBuf::from(s.as_str()), <_ as AsRef<Path>>::as_ref(s));
        assert_eq!(Path::new(s.as_str()), <_ as AsRef<Path>>::as_ref(s));

        assert_eq!(s.as_bytes(), <_ as AsRef<[u8]>>::as_ref(s));
        assert_eq!(s.as_str(), <_ as AsRef<str>>::as_ref(s));
        assert_eq!(s.as_str(), <_ as Borrow<str>>::borrow(s));

        assert_eq!(s, s.as_str());
        assert_eq!(*s, std::string::String::from(s.as_str()));
        assert_eq!(*s, std::string::String::from(s));
        assert_eq!(*s, std::string::String::from(s.clone()));
        assert_eq!(*s, RcString::from(std::string::String::from(s).as_mut()));
        assert_eq!(*s, RcString::from(std::string::String::from(s)));
        assert_eq!(*s, RcString::from(&std::string::String::from(s)));
        assert_eq!(*s, RcString::from(s.as_str()));
        assert_eq!(*s, ArcString::from(s.as_str()));
        assert_eq!(*s, RcString::from(Cow::from(s.as_str())));
        assert_eq!(*s, RcString::from(Cow::from(s.as_str().to_owned())));
        assert_eq!(*s, RcString::from(s));
        assert_eq!(*s, Cow::from(s.as_str()));
    }
}

#[test]
fn rc_string() {
    test_string::<Cell<usize>>();
}

#[test]
fn arc_string() {
    test_string::<AtomicUsize>();
}
