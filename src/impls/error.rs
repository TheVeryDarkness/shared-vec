use crate::{Counter, String};
use alloc::boxed::Box;
use core::{error::Error, fmt};

struct StringError<C: Counter<usize>>(String<C>);

impl<C: Counter<usize>> Error for StringError<C> {}

impl<C: Counter<usize>> fmt::Display for StringError<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

// Purposefully skip printing "StringError(..)"
impl<C: Counter<usize>> fmt::Debug for StringError<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<'a, C: Counter<usize> + Send + Sync + 'a> From<String<C>>
    for Box<dyn Error + Send + Sync + 'a>
{
    #[inline]
    fn from(err: String<C>) -> Self {
        Box::new(StringError(err))
    }
}
impl<'a, C: Counter<usize> + Send + Sync + 'a> From<String<C>> for Box<dyn Error + Send + 'a> {
    #[inline]
    fn from(err: String<C>) -> Self {
        Box::new(StringError(err))
    }
}
impl<'a, C: Counter<usize> + 'a> From<String<C>> for Box<dyn Error + 'a> {
    #[inline]
    fn from(err: String<C>) -> Self {
        Box::new(StringError(err))
    }
}
