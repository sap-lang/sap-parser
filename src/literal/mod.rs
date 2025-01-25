pub mod array;
pub mod number;
pub mod object;
pub mod string;

use array::ArrayBody;
use number::SapNumber;
use object::ObjectBody;
use pest_ast::FromPest;
use serde::Serialize;
use string::SapString;

use crate::Rule;

fn parse_bool(span: pest::Span) -> bool {
    let str = span.as_str();
    match str {
        "true" => true,
        "false" => false,
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, PartialEq, FromPest)]
#[pest_ast(rule(Rule::boolean))]
pub struct Boolean {
    #[pest_ast(outer(with(parse_bool)))]
    pub value: bool,
}

impl Serialize for Boolean {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bool(self.value)
    }
}

#[derive(Debug, Clone, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::void))]
pub struct Void;

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::literal))]
pub enum Literal {
    Boolean(Boolean),
    Void(Void),
    String(SapString),
    Number(SapNumber),
}

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::compound_literal))]
pub enum CompoundLiteral {
    ArrayLiteral(ArrayBody),
    ObjectLiteral(ObjectBody),
    Literal(Literal),
}

#[cfg(test)]
mod tests {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::Rule;

    #[test]
    fn test_void() {
        let pair = crate::SapParser::parse(Rule::void, "()")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = pest::iterators::Pairs::single(pair.clone());
        let void = super::Void::from_pest(&mut pairs).unwrap();
        assert_eq!(void, super::Void);
    }
}
