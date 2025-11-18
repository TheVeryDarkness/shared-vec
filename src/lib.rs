#![no_std]
extern crate alloc;

use core::{cell::Cell, sync::atomic};

mod counter;
mod inner;
mod string;
mod vec;

pub use counter::Counter;
pub use string::String;
pub use vec::Vec;

pub type RcVec<T> = Vec<Cell<usize>, T>;
pub type ArcVec<T> = Vec<atomic::AtomicUsize, T>;
pub type RcString = String<Cell<usize>>;
pub type ArcString = String<atomic::AtomicUsize>;
