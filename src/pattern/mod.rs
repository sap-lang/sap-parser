pub mod array;
pub mod object;

use array::ArrayPattern;
use object::ObjectPattern;
use pest_ast::FromPest;
use serde::Serialize;

use crate::{
    Rule,
    id::{Id, MacroId},
    literal::Literal,
};

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::eclipse_pattern))]
pub struct EclipsePattern {
    pub value: Id,
}

#[derive(Debug, Clone, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::pattern))]
pub enum Pattern {
    Id(Id),
    Literal(Literal),
    ArrayPattern(ArrayPattern),
    ObjectPattern(ObjectPattern),
}

#[cfg(test)]
mod tests {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::Rule;

    #[test]
    fn test_eclipse_pattern() {
        let pair = crate::SapParser::parse(Rule::eclipse_pattern, "...a")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = pest::iterators::Pairs::single(pair.clone());
        let eclipse_pattern = crate::pattern::EclipsePattern::from_pest(&mut pairs).unwrap();
        assert_eq!(eclipse_pattern.value.value(), "a");
    }
}
