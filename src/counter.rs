//! Counter trait and its implementations for various integer types.
//!
//! See [alloc::sync::Arc].

use core::{
    cell::Cell,
    sync::atomic,
    sync::atomic::{AtomicU16, AtomicU32, AtomicU64, AtomicU8, AtomicUsize, Ordering},
};

/// Counter trait for uniformed reference counting.
///
/// # Safety
///
/// The implementation must hold arithmetic invariants and respect synchronization.
pub unsafe trait Counter<T>: Sized {
    unsafe fn increment(&self);
    unsafe fn decrement(&self) -> bool;
    fn fence_acquire();
    fn one() -> Self;
}

macro_rules! impl_cell {
    ($ty:ty) => {
        unsafe impl Counter<$ty> for Cell<$ty> {
            unsafe fn increment(&self) {
                self.set(self.get() + 1);
            }

            unsafe fn decrement(&self) -> bool {
                let new = self.get() - 1;
                self.set(new);
                new == 0
            }

            fn fence_acquire() {}

            fn one() -> Self {
                Cell::new(1)
            }
        }
    };
}

impl_cell!(u8);
impl_cell!(u16);
impl_cell!(u32);
impl_cell!(u64);
impl_cell!(usize);

macro_rules! impl_atomic {
    ($ty:ty, $equiv:ty) => {
        unsafe impl Counter<usize> for $ty {
            unsafe fn increment(&self) {
                self.fetch_add(1, Ordering::Relaxed);
            }

            unsafe fn decrement(&self) -> bool {
                let old = self.fetch_sub(1, Ordering::Release);
                debug_assert!(old != 0);
                old == 1
            }

            fn fence_acquire() {
                atomic::fence(Ordering::Acquire);
            }

            fn one() -> Self {
                Self::new(1)
            }
        }
    };
}

impl_atomic!(AtomicU8, u8);
impl_atomic!(AtomicU16, u16);
impl_atomic!(AtomicU32, u32);
impl_atomic!(AtomicU64, u64);
impl_atomic!(AtomicUsize, usize);
