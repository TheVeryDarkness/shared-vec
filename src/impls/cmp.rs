use crate::{Counter, String, Vec};
use alloc::borrow::Cow;
use core::cmp::Ordering;

macro_rules! impl_eq_as_str {
    ($lhs:ty, $rhs:ty) => {
        impl<C: Counter<usize>> PartialEq<$rhs> for $lhs {
            fn eq(&self, other: &$rhs) -> bool {
                let lhs: &str = self;
                let rhs: &str = other;
                lhs == rhs
            }
        }
    };
}

impl_eq_as_str!(String<C>, Cow<'_, str>);
impl_eq_as_str!(Cow<'_, str>, String<C>);

impl_eq_as_str!(String<C>, str);
impl_eq_as_str!(str, String<C>);

impl_eq_as_str!(String<C>, alloc::string::String);
impl_eq_as_str!(alloc::string::String, String<C>);

impl<C1: Counter<usize>, C2: Counter<usize>> PartialEq<String<C2>> for String<C1> {
    fn eq(&self, other: &String<C2>) -> bool {
        let lhs: &str = self;
        let rhs: &str = other;
        lhs == rhs
    }
}
impl<C: Counter<usize>> Eq for String<C> {}
impl<C1: Counter<usize>, C2: Counter<usize>> PartialOrd<String<C2>> for String<C1> {
    fn partial_cmp(&self, other: &String<C2>) -> Option<Ordering> {
        Some(self.as_str().cmp(other.as_str()))
    }
}
impl<C: Counter<usize>> Ord for String<C> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(&other.as_str())
    }
}

macro_rules! impl_eq_as_slice {
    ($lhs:ty, $rhs:ty $(; $($tt:tt)+ )? ) => {
        impl<C: Counter<usize>, T: PartialEq $(, $($tt)+)? > PartialEq<$rhs> for $lhs {
            fn eq(&self, other: &$rhs) -> bool {
                let lhs: &[T] = self;
                let rhs: &[T] = other;
                lhs == rhs
            }
        }
    };
}

impl_eq_as_slice!(Vec<C, T>, [T; N]; const N: usize);
impl_eq_as_slice!([T; N], Vec<C, T>; const N: usize);

impl_eq_as_slice!(Vec<C, T>, [T]);
impl_eq_as_slice!([T], Vec<C, T>);

impl_eq_as_slice!(Vec<C, T>, Vec<C, T>);

impl<C: Counter<usize>, T> Eq for Vec<C, T> where T: Eq {}
impl<C: Counter<usize>, T> PartialOrd for Vec<C, T>
where
    T: PartialOrd,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}
impl<C: Counter<usize>, T> Ord for Vec<C, T>
where
    T: Ord,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}
