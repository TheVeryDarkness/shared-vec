use random_string::generate_rng;
use shared_vec::Counter;
use std::cell::Cell;
use std::iter;
use std::sync::atomic::AtomicUsize;

const CHARSET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789🦀";

fn test_string<C: Counter<usize>>() {
    let mut strings = iter::repeat_with(|| generate_rng(0..1024, CHARSET))
        .take(1024)
        .map(|s| shared_vec::String::<C>::from_boxed_str(s.into_boxed_str()))
        .collect::<Vec<shared_vec::String<C>>>();

    strings.sort_by(|a, b| a.as_str().cmp(b.as_str()));
    assert!(strings.is_sorted());
}

#[test]
fn rc_string() {
    test_string::<Cell<usize>>();
}

#[test]
fn arc_string() {
    test_string::<AtomicUsize>();
}
