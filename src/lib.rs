#![feature(box_patterns)]

pub mod formatter;
pub mod preprocessor;

pub mod expr;
pub mod function;
pub mod id;
pub mod literal;
pub mod operator_precedence;
pub mod pattern;
pub mod quotations;

use pest_derive::Parser;

pub fn span_to_string(span: pest::Span) -> String {
    span.as_str().to_string()
}

#[derive(Parser)]
#[grammar = "parser.pest"]
pub struct SapParser;
