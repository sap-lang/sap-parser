pub mod array;
pub mod object;

use array::ArrayPattern;
use object::ObjectPattern;
use pest_ast::FromPest;
use serde::Serialize;

use crate::{
    Rule,
    diagnostics::Diagnostic,
    id::Id,
    literal::Literal,
};

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::eclipse_pattern))]
pub struct EclipsePattern {
    #[pest_ast(outer(with(Diagnostic::from_span)))]
    pub diag: Diagnostic,
    pub value: Id,
}

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
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

    #[test]
    fn test_pattern() {
        let pair = crate::SapParser::parse(Rule::pattern, "^{c: ^[a,...b]")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = pest::iterators::Pairs::single(pair.clone());
        let pattern = crate::pattern::Pattern::from_pest(&mut pairs).unwrap();
        println!("{:#?}", pattern);
    }
}
