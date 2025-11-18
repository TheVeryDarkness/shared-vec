use crate::{counter::Counter, Vec};
use alloc::boxed::Box;
use core::{
    fmt,
    hash::Hash,
    ops::{Deref, RangeBounds},
    str::Utf8Error,
};

pub struct String<C: Counter<usize>> {
    vec: Vec<C, u8>,
}

impl<C: Counter<usize>> String<C> {
    /// Create a new empty `String`.
    #[inline]
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }

    /// Create a `String` from a `Box<[u8]>`
    ///
    /// # Errors
    ///
    /// Returns an error if the bytes are not valid UTF-8.
    #[inline]
    pub fn from_utf8(bytes: Box<[u8]>) -> Result<Self, Utf8Error> {
        str::from_utf8(&bytes)?;
        Ok(Self {
            vec: Vec::from_boxed_slice(bytes),
        })
    }

    /// Create a `String` from a `Box<[u8]>` without checking for UTF-8 validity.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the bytes are valid UTF-8.
    #[inline]
    pub unsafe fn from_utf8_unchecked(bytes: Box<[u8]>) -> Self {
        Self {
            vec: Vec::from_boxed_slice(bytes),
        }
    }

    /// Create a `String` from a `Box<str>`
    #[inline]
    pub fn from_boxed_str(bytes: Box<str>) -> Self {
        unsafe { Self::from_utf8_unchecked(bytes.into_boxed_bytes()) }
    }
}

impl<C: Counter<usize>> Default for String<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: Counter<usize>> Clone for String<C> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            vec: self.vec.clone(),
        }
    }
}

impl<C: Counter<usize>> Deref for String<C> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { str::from_utf8_unchecked(&self.vec) }
    }
}

impl<C: Counter<usize>> fmt::Debug for String<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <str as fmt::Debug>::fmt(&**self, f)
    }
}

impl<C: Counter<usize>> String<C> {
    /// Returns the length of the string.
    #[inline]
    pub const fn len(&self) -> usize {
        self.vec.len()
    }

    /// Returns true if the vector is empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    /// Returns a slice of the vector.
    #[inline]
    pub fn bytes(&self) -> &[u8] {
        &self.vec
    }

    /// Returns a string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        self
    }

    /// Return the start and length of the range without checking.
    #[inline]
    pub(crate) fn convert_range_unchecked(&self, range: impl RangeBounds<usize>) -> (usize, usize) {
        self.vec.convert_range_unchecked(range)
    }

    /// Validate the range and return the start and length.
    #[inline]
    pub(crate) fn validate_range(&self, range: impl RangeBounds<usize>) -> Option<(usize, usize)> {
        let cap = self.len();
        let start = match range.start_bound() {
            core::ops::Bound::Included(&s) => s,
            core::ops::Bound::Excluded(&s) => s.checked_add(1)?,
            core::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            core::ops::Bound::Included(&e) => e.checked_add(1)?,
            core::ops::Bound::Excluded(&e) => e,
            core::ops::Bound::Unbounded => cap,
        };
        if end > cap {
            return None;
        }
        let s = self.as_str();
        if !s.is_char_boundary(start) || !s.is_char_boundary(end) {
            return None;
        }
        (end.checked_sub(start)).map(|len| (start, len))
    }

    /// Check if the range is valid.
    #[inline]
    pub fn is_valid_range(&self, range: impl RangeBounds<usize>) -> bool {
        self.validate_range(range).is_some()
    }

    /// Get a sub-slice of the vector.
    #[inline]
    pub fn get(&self, range: impl RangeBounds<usize>) -> Option<Self> {
        let (start, len) = self.validate_range(range)?;
        Some(Self {
            vec: unsafe { self.vec.slice(start, len) },
        })
    }

    /// Get a sub-slice of the vector.
    ///
    /// # Panics
    ///
    /// Panics if the range is invalid.
    #[inline]
    pub fn idx(&self, range: impl RangeBounds<usize>) -> Self {
        self.get(range).expect("Invalid range")
    }

    /// Get a sub-slice of the vector without checking the range.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the range is valid.
    #[inline]
    pub unsafe fn get_unchecked(&self, range: impl RangeBounds<usize>) -> Self {
        let (start, len) = self.convert_range_unchecked(range);
        let vec = self.vec.slice(start, len);
        Self { vec }
    }
}

impl<C: Counter<usize>> From<Box<str>> for String<C> {
    #[inline]
    fn from(s: Box<str>) -> Self {
        Self::from_boxed_str(s)
    }
}

impl<C: Counter<usize>> PartialEq for String<C> {
    fn eq(&self, other: &Self) -> bool {
        self.vec == other.vec
    }
}
impl<C: Counter<usize>> Eq for String<C> {}
impl<C: Counter<usize>> PartialOrd for String<C> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<C: Counter<usize>> Ord for String<C> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.vec.cmp(&other.vec)
    }
}
impl<C: Counter<usize>> Hash for String<C> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.vec.hash(state);
    }
}
