use crate::{Counter, String, Vec};
use core::hash::Hash;

impl<C: Counter<usize>> Hash for String<C> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
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
