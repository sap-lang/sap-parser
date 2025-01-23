pub mod array;
pub mod object;

use array::ArrayPattern;
use from_pest::FromPest;
use object::ObjectPattern;
use pest_ast::FromPest;
use serde::Serialize;

use crate::{Id, MacroId, Rule, literal::Literal};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EclipsePattern {
    pub value: Id,
}

impl FromPest<'_> for EclipsePattern {
    type Rule = Rule;

    type FatalError = from_pest::Void;

    fn from_pest(
        pest: &mut pest::iterators::Pairs<'_, Self::Rule>,
    ) -> Result<Self, from_pest::ConversionError<Self::FatalError>> {
        let pair = pest.next().unwrap();
        let str = pair.as_str();
        Ok(EclipsePattern {
            value: Id::NormalId(crate::NormalId {
                value: str[3..].to_string(),
            }),
        })
    }
}

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::macro_pattern))]
pub struct MacroPattern {
    pub macro_name: MacroId,
    pub pattern: Box<Pattern>,
}

#[derive(Debug, Clone, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::pattern))]
pub enum Pattern {
    Id(Id),
    Literal(Literal),
    ArrayPattern(ArrayPattern),
    ObjectPattern(ObjectPattern),
    MacroPattern(MacroPattern),
}

#[cfg(test)]
mod test {
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
