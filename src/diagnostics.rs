use core::fmt;

use pest::Span;
use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub struct Diagnostic {
    pub start_line: usize,
    pub start_col: usize,
    pub start_offset: usize,

    pub end_line: usize,
    pub end_col: usize,
    pub end_offset: usize,

    pub source_code: &'static str,
}

impl PartialEq for Diagnostic {
    fn eq(&self, other: &Self) -> bool {
        if self.end_offset == 0 || other.end_offset == 0 {
            // test
            true
        } else {
            self.start_line == other.start_line
                && self.start_col == other.start_col
                && self.start_offset == other.start_offset
                && self.end_line == other.end_line
                && self.end_col == other.end_col
                && self.end_offset == other.end_offset
                && self.source_code == other.source_code
        }
    }
}

impl fmt::Debug for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}..{}", self.start_offset, self.end_offset))
    }
}

impl Diagnostic {
    pub fn from_span(span: Span<'_>) -> Self {
        let (start_line, start_col) = span.start_pos().line_col();
        let (end_line, end_col) = span.end_pos().line_col();
        let start_offset = span.start();
        let end_offset = span.end();
        let source_code = span.as_str();
        let source_code = unsafe { std::mem::transmute::<&str, &'static str>(source_code) };
        Self {
            start_line,
            start_col,
            start_offset,
            end_line,
            end_col,
            end_offset,
            source_code,
        }
    }

    pub fn test() -> Self {
        Diagnostic {
            start_line: 0,
            start_col: 0,
            start_offset: 0,
            end_line: 0,
            end_col: 0,
            end_offset: 0,
            source_code: "",
        }
    }
}
