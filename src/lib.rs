#![no_std]
#![doc = include_str!("../README.md")]

extern crate alloc;

use core::{cell::Cell, sync::atomic};

mod counter;
mod inner;
mod string;
mod vec;
mod impls;

pub use counter::Counter;
pub use string::String;
pub use vec::Vec;

/// Type alias for a reference-counted [Vec] using [`Cell<usize>`] as the counter.
pub type RcVec<T> = Vec<Cell<usize>, T>;
/// Type alias for an atomically reference-counted [Vec] using [`atomic::AtomicUsize`] as the counter.
pub type ArcVec<T> = Vec<atomic::AtomicUsize, T>;
/// Type alias for a reference-counted [String] using [`Cell<usize>`] as the counter.
pub type RcString = String<Cell<usize>>;
/// Type alias for an atomically reference-counted [String] using [`atomic::AtomicUsize`] as the counter.
pub type ArcString = String<atomic::AtomicUsize>;
