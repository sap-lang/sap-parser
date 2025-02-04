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

#[macro_export]
macro_rules! ast_with_diagnostic {
    ($name:ident ($rule:ident) {
        $($variant:ident($($arg:ident: $ty:ty),*)),* $(,)?
    }) => {
        use $crate::diagnostics::Diagnostic;
        use from_pest::FromPest;
        use pest::iterators::Pairs;

        #[derive(Debug, Clone, PartialEq, Serialize)]
        pub struct $name {
            pub inner: Inner,
            pub diag: Diagnostic,
        }

        impl $name {
            $(
                #[allow(non_snake_case)]
                pub fn $variant($($arg: $ty),*, diag: Diagnostic) -> Self {
                    Self {
                        inner: Inner::$variant($($arg),*),
                        diag,
                    }
                }
            )*
        }

        impl FromPest<'_> for $name {
            type Rule = Rule;

            type FatalError = from_pest::Void;

            fn from_pest(
                pest: &mut pest::iterators::Pairs<'_, Self::Rule>,
            ) -> Result<Self, from_pest::ConversionError<Self::FatalError>> {
                let pest = pest.next();
                if pest.is_none() {
                    return Err(from_pest::ConversionError::NoMatch);
                }
                let pest = pest.unwrap();

                let diag = Diagnostic::from_span(pest.as_span());
                let inner = Inner::from_pest(&mut Pairs::single(pest))?;
                Ok($name { inner, diag })
            }
        }

        #[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
        #[pest_ast(rule(Rule::$rule))]
        pub enum Inner {
            $(
                $variant($($ty),*),
            )*
        }
    };
}
