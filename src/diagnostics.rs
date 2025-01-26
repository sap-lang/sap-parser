use core::fmt;

use pest::Span;
use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Serialize)]
pub struct Diagnostic {
    pub start_line: usize,
    pub start_col: usize,
    pub start_offset: usize,

    pub end_line: usize,
    pub end_col: usize,
    pub end_offset: usize,
}

impl fmt::Debug for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}..{}", self.start_offset, self.end_offset))
    }
}

impl Diagnostic {
    pub fn from_span(span: &Span<'_>) -> Self {
        let (start_line, start_col) = span.start_pos().line_col();
        let (end_line, end_col) = span.end_pos().line_col();
        let start_offset = span.start();
        let end_offset = span.end();
        Self {
            start_line,
            start_col,
            start_offset,
            end_line,
            end_col,
            end_offset,
        }
    }
}
