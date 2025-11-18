use crate::{counter::Counter, inner::Inner};
use alloc::boxed::Box;
use core::ops::RangeBounds;
use core::{fmt, ptr};
use core::{marker::PhantomData, ops::Deref, ptr::NonNull};

pub struct Vec<C: Counter<usize>, T> {
    inner: Inner<C, [T]>,
    ptr: NonNull<[T]>,
    phantom: PhantomData<Inner<C, [T]>>,
}

unsafe impl<C: Counter<usize>, T> Send for Vec<C, T>
where
    C: Send + Sync,
    T: Send + Sync,
{
}
unsafe impl<C: Counter<usize>, T> Sync for Vec<C, T>
where
    C: Send + Sync,
    T: Send + Sync,
{
}

impl<C: Counter<usize>, T> Vec<C, T> {
    #[inline]
    unsafe fn from_inner(inner: Inner<C, [T]>) -> Self {
        Self {
            ptr: inner.data,
            inner,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn from_boxed_slice(data: Box<[T]>) -> Self {
        let x: Inner<C, [T]> = Inner {
            ref_count: Box::leak(Box::new(C::one())).into(),
            data: Box::leak(data).into(),
        };
        unsafe { Self::from_inner(x) }
    }
    #[inline]
    pub fn new() -> Self {
        let x: Inner<C, [T]> = Inner {
            ref_count: Box::leak(Box::new(C::one())).into(),
            data: Box::leak(Box::<[T; 0]>::new([]) as Box<[T]>).into(),
        };
        unsafe { Self::from_inner(x) }
    }
}

impl<C: Counter<usize>, T> Clone for Vec<C, T> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            let ref_count = self.inner.ref_count.as_ref();
            ref_count.increment();
        }
        Self {
            inner: self.inner,
            ptr: self.ptr,
            phantom: PhantomData,
        }
    }
}

impl<C: Counter<usize>, T> Drop for Vec<C, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let ref_count = self.inner.ref_count.as_ref();
            if ref_count.decrement() {
                C::fence_acquire();
                let ref_count = Box::from_raw(self.inner.ref_count.as_ptr());
                let data = Box::from_raw(self.inner.data.as_ptr());
                drop(ref_count);
                drop(data);
            }
        }
    }
}

impl<C: Counter<usize>, T> Default for Vec<C, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: Counter<usize>, T> Deref for Vec<C, T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<C: Counter<usize>, T> fmt::Debug for Vec<C, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <[T] as fmt::Debug>::fmt(&**self, f)
    }
}

impl<C: Counter<usize>, T> Vec<C, T> {
    /// Returns the length of the vector.
    #[inline]
    pub const fn len(&self) -> usize {
        unsafe { self.ptr.as_ref() }.len()
    }

    /// Returns true if the vector is empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a slice of the vector.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self
    }

    /// Return the start and length of the range without checking.
    #[inline]
    pub(crate) fn convert_range_unchecked(&self, range: impl RangeBounds<usize>) -> (usize, usize) {
        let start = match range.start_bound() {
            core::ops::Bound::Included(&s) => s,
            core::ops::Bound::Excluded(&s) => s + 1,
            core::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            core::ops::Bound::Included(&e) => e + 1,
            core::ops::Bound::Excluded(&e) => e,
            core::ops::Bound::Unbounded => self.len(),
        };
        (start, end - start)
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
        (end.checked_sub(start)).map(|len| (start, len))
    }

    /// Check if the range is valid.
    #[inline]
    pub fn is_valid_range(&self, range: impl RangeBounds<usize>) -> bool {
        self.validate_range(range).is_some()
    }

    pub(crate) unsafe fn slice(&self, start: usize, len: usize) -> Self {
        let ptr: *mut T = self.inner.data.as_ptr().cast();
        let ptr = unsafe { ptr.add(start) };
        let ptr = ptr::slice_from_raw_parts_mut(ptr, len);
        let ptr = unsafe { NonNull::new_unchecked(ptr) };

        self.inner.ref_count.as_ref().increment();
        Self {
            inner: self.inner,
            ptr,
            phantom: PhantomData,
        }
    }

    /// Get a sub-slice of the vector.
    #[inline]
    pub fn get(&self, range: impl RangeBounds<usize>) -> Option<Self> {
        let (start, len) = self.validate_range(range)?;
        Some(unsafe { self.slice(start, len) })
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
        self.slice(start, len)
    }
}

impl<C: Counter<usize>, T> From<Box<[T]>> for Vec<C, T> {
    #[inline]
    fn from(data: Box<[T]>) -> Self {
        Self::from_boxed_slice(data)
    }
}

impl<C: Counter<usize>, T> PartialEq for Vec<C, T>
where
    T: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}
impl<C: Counter<usize>, T> Eq for Vec<C, T> where T: Eq {}
impl<C: Counter<usize>, T> PartialOrd for Vec<C, T>
where
    T: PartialOrd,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}
impl<C: Counter<usize>, T> Ord for Vec<C, T>
where
    T: Ord,
{
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}
impl<C: Counter<usize>, T> core::hash::Hash for Vec<C, T>
where
    T: core::hash::Hash,
{
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}
