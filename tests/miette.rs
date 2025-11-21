//! Tests for the miette integration.

#![cfg(feature = "miette")]

use miette::{
    Diagnostic, GraphicalReportHandler, GraphicalTheme, LabeledSpan, NamedSource, SourceCode,
    SourceSpan,
};
use shared_vec::ArcString;
use std::{error, fmt};

struct TestCase {
    error: ArcString,
    input: NamedSource<ArcString>,
    span: SourceSpan,
}

impl fmt::Debug for TestCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl fmt::Display for TestCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl error::Error for TestCase {}

impl Diagnostic for TestCase {
    fn source_code(&self) -> Option<&dyn SourceCode> {
        Some(&self.input)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        Some(Box::new(
            [LabeledSpan::new(
                Some("error occurred here".into()),
                self.span.offset(),
                self.span.len(),
            )]
            .into_iter(),
        ))
    }
}

#[test]
fn test_miette_integration() {
    let test_case = TestCase {
        error: ArcString::from(
            "The second line has an error. The sentence is not terminated by a period.",
        ),
        input: NamedSource::new(
            "tests/miette.rs",
            ArcString::from(
                "\
This is a test input string.
Let's check if the input string misses something
End of string.",
            ),
        ),
        span: SourceSpan::new(77.into(), 1),
    };

    let mut report_string = String::new();

    GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor())
        .render_report(&mut report_string, &test_case)
        .unwrap();

    assert_eq!(
        report_string,
        "  × The second line has an error. The sentence is not terminated by a period.
   ╭─[tests/miette.rs:2:49]
 1 │ This is a test input string.
 2 │ Let's check if the input string misses something
   ·                                                 ┬
   ·                                                 ╰── error occurred here
 3 │ End of string.
   ╰────
",
        "\n{report_string}",
    );
}
