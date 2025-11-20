use crate::{Counter, String, Vec};
use core::borrow::Borrow;

impl<C: Counter<usize>> Borrow<str> for String<C> {
    #[inline]
    fn borrow(&self) -> &str {
        self
    }
}
impl<C: Counter<usize>, T> Borrow<[T]> for Vec<C, T> {
    #[inline]
    fn borrow(&self) -> &[T] {
        self
    }
}
