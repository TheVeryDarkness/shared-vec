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
