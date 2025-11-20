#![cfg(feature = "serde")]

use crate::{Counter, String, Vec};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl<C: Counter<usize>, T: Serialize> Serialize for Vec<C, T> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        <[T] as Serialize>::serialize(self, serializer)
    }
}

impl<'de, C: Counter<usize>, T: Deserialize<'de>> Deserialize<'de> for Vec<C, T> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = alloc::vec::Vec::<T>::deserialize(deserializer)?;
        Ok(Self::from(v.into_boxed_slice()))
    }
}

impl<C: Counter<usize>> Serialize for String<C> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        str::serialize(self, serializer)
    }
}

impl<'de, C: Counter<usize>> Deserialize<'de> for String<C> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = alloc::string::String::deserialize(deserializer)?;
        Ok(Self::from_boxed_str(s.into_boxed_str()))
    }
}
