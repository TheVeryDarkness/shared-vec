#![cfg(feature = "miette")]

use crate::{Counter, String};
use alloc::boxed::Box;
use miette::{MietteError, SourceCode, SourceSpan, SpanContents};

impl<C: Counter<usize> + Send + Sync> SourceCode for String<C> {
    #[inline]
    fn read_span<'a>(
        &'a self,
        span: &SourceSpan,
        context_lines_before: usize,
        context_lines_after: usize,
    ) -> Result<Box<dyn SpanContents<'a> + 'a>, MietteError> {
        <str as SourceCode>::read_span(self, span, context_lines_before, context_lines_after)
    }
}
