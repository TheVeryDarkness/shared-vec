use itertools::Itertools;
use shared_vec::Counter;
use std::cell::Cell;
use std::sync::atomic::AtomicUsize;

fn test_string<C: Counter<usize>>() {
    let mut strings = [
        'a', 'b', '¥', '±', 'α', 'β', '₍', '₎', '₤', '€', '⻰', '⻳', '🦀', '🧐', '𝆒', '𝆓',
    ]
    .iter()
    .copied()
    .permutations(3)
    .map(|s| shared_vec::String::<C>::from_boxed_str(String::from_iter(s).into_boxed_str()))
    .collect::<Vec<shared_vec::String<C>>>();

    strings.sort_by(|a, b| a.as_str().cmp(b.as_str()));
    assert!(strings.iter().all(|w| w.chars().count() == 3));
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
