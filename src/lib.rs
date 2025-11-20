#![no_std]
#![doc = include_str!("../README.md")]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use core::cell::Cell;
use core::sync::atomic::AtomicUsize;

mod counter;
mod impls;
mod inner;
mod string;
mod vec;

pub use counter::Counter;
pub use string::String;
pub use vec::Vec;

/// Type alias for a reference-counted [Vec] using [`Cell<usize>`] as the counter.
pub type RcVec<T> = Vec<Cell<usize>, T>;
/// Type alias for an atomically reference-counted [Vec] using [`AtomicUsize`] as the counter.
pub type ArcVec<T> = Vec<AtomicUsize, T>;
/// Type alias for a reference-counted [String] using [`Cell<usize>`] as the counter.
pub type RcString = String<Cell<usize>>;
/// Type alias for an atomically reference-counted [String] using [`AtomicUsize`] as the counter.
pub type ArcString = String<AtomicUsize>;
