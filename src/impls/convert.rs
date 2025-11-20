use crate::{Counter, String, Vec};
use alloc::borrow::Cow;
use alloc::boxed::Box;

impl<C: Counter<usize>> From<Box<str>> for String<C> {
    /// This takes ownership of `s` and creates a `String` without copying.
    #[inline]
    fn from(s: Box<str>) -> Self {
        Self::from_boxed_str(s)
    }
}
impl<C: Counter<usize>> From<&str> for String<C> {
    /// The resulting `String` will contain a copy of `s` on the heap.
    #[inline]
    fn from(s: &str) -> Self {
        Self::from_boxed_str(s.into())
    }
}
impl<C: Counter<usize>> From<&mut str> for String<C> {
    /// The resulting `String` will contain a copy of `s` on the heap.
    #[inline]
    fn from(s: &mut str) -> Self {
        Self::from_boxed_str(s.into())
    }
}
impl<C1: Counter<usize>, C2: Counter<usize>> From<&String<C2>> for String<C1> {
    /// This creates a `String` with counter `C1` containing a copy of the data in `s` on the heap.
    #[inline]
    fn from(s: &String<C2>) -> Self {
        Self::from_boxed_str(s.as_str().into())
    }
}
impl<C: Counter<usize>> From<&alloc::string::String> for String<C> {
    /// The resulting `String` will contain a copy of `s` on the heap.
    #[inline]
    fn from(s: &alloc::string::String) -> Self {
        Self::from(s.as_str())
    }
}
impl<C: Counter<usize>> From<alloc::string::String> for String<C> {
    /// This takes ownership of `s`, shrinks it with [`alloc::string::String::into_boxed_str`],
    /// and creates a `String` without copying.
    #[inline]
    fn from(s: alloc::string::String) -> Self {
        Self::from_boxed_str(s.into_boxed_str())
    }
}
impl<C: Counter<usize>> From<&String<C>> for alloc::string::String {
    /// This creates an owned `alloc::string::String` by copying the data from `s`.
    #[inline]
    fn from(s: &String<C>) -> Self {
        alloc::string::String::from(s.as_str())
    }
}
impl<C: Counter<usize>> From<String<C>> for alloc::string::String {
    /// This creates an owned `alloc::string::String` by copying the data from `s`.
    #[inline]
    fn from(s: String<C>) -> Self {
        alloc::string::String::from(s.as_str())
    }
}

impl<C: Counter<usize>> From<Cow<'_, str>> for String<C> {
    /// This creates a `String` from either a borrowed or owned string.
    ///
    /// - If the `Cow` is borrowed, it will copy the data to the heap.
    /// - If the `Cow` is owned, it will take ownership of the data without copying.
    #[inline]
    fn from(s: Cow<'_, str>) -> Self {
        match s {
            Cow::Borrowed(s) => Self::from_boxed_str(s.into()),
            Cow::Owned(s) => Self::from_boxed_str(s.into_boxed_str()),
        }
    }
}

impl<C: Counter<usize>, T> From<Box<[T]>> for Vec<C, T> {
    #[inline]
    fn from(data: Box<[T]>) -> Self {
        Self::from_boxed_slice(data)
    }
}
