#![feature(box_patterns)]

pub mod formatter;
pub mod preprocessor;

pub mod diagnostics;
pub mod expr;
pub mod function;
pub mod id;
pub mod literal;
pub mod operator_precedence;
pub mod pattern;
pub mod quotations;

use from_pest::FromPest;
use pest::Parser;
use pest_derive::Parser;

pub fn span_to_string(span: pest::Span) -> String {
    span.as_str().to_string()
}

#[derive(Parser)]
#[grammar = "parser.pest"]
pub struct SapParser;

pub fn parse_expr(input: &str) -> Result<expr::Expr, from_pest::ConversionError<from_pest::Void>> {
    let pair = crate::SapParser::parse(Rule::expr, input)
        .unwrap()
        .next()
        .unwrap();
    let mut pairs = pest::iterators::Pairs::single(pair);
    expr::Expr::from_pest(&mut pairs)
}
