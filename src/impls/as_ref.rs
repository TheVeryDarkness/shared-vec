use crate::{Counter, String, Vec};

impl<C: Counter<usize>> AsRef<str> for String<C> {
    #[inline]
    fn as_ref(&self) -> &str {
        self
    }
}
impl<C: Counter<usize>> AsRef<[u8]> for String<C> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<C: Counter<usize>, T> AsRef<[T]> for Vec<C, T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self
    }
}
