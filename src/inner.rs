use core::ptr::NonNull;

/// Inner data structure for [`Shared`]
///
/// Holds the actual data and reference count
///
/// See [`Shared`] for more details.
///
/// Copied from [alloc::sync::Arc].
///
/// [`Shared`]: crate::Shared
pub(crate) struct Inner<C, T: ?Sized> {
    pub(crate) ref_count: NonNull<C>,
    pub(crate) data: NonNull<T>,
}

impl<C, T: ?Sized> Clone for Inner<C, T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<C, T: ?Sized> Copy for Inner<C, T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ptr;

    #[test]
    fn clone_inner() {
        use core::cell::Cell;
        let data = [1, 2, 3];
        let cell = Cell::new(1u32);
        let inner = Inner {
            ref_count: NonNull::new(ptr::from_ref(&cell).cast_mut()).unwrap(),
            data: NonNull::new(ptr::from_ref(&data).cast_mut()).unwrap(),
        };
        let inner2 = inner.clone();
        assert_eq!(inner.ref_count, inner2.ref_count);
        assert_eq!(inner.data, inner2.data);
    }
}
