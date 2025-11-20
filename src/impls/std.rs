#![cfg(feature = "std")]

use crate::{Counter, String};

impl<C: Counter<usize>> AsRef<std::path::Path> for String<C> {
    #[inline]
    fn as_ref(&self) -> &std::path::Path {
        std::path::Path::new(self.as_str())
    }
}
