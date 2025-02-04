use pest_ast::FromPest;
use serde::Serialize;

use crate::{Rule, diagnostics::Diagnostic};

use super::{EclipsePattern, Pattern};

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::array_pattern_elem))]
pub enum ArrayPatternElem {
    EclipsePattern(EclipsePattern),
    Pattern(Pattern),
}

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::array_pattern_body))]
pub struct ArrayPatternBody {
    pub elems: Vec<ArrayPatternElem>,
}

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::array_pattern))]
pub struct ArrayPattern {
    #[pest_ast(outer(with(Diagnostic::from_span)))]
    pub diag: Diagnostic,
    pub body: ArrayPatternBody,
}

#[cfg(test)]
mod tests {
    use from_pest::FromPest;
    use pest::Parser;

    use super::*;
    use crate::{
        Rule, SapParser,
        diagnostics::Diagnostic,
        id::{Id, NormalId},
    };

    #[test]
    fn test_array_pattern_elem() {
        let pair = SapParser::parse(Rule::array_pattern_elem, "...a")
            .unwrap()
            .next()
            .unwrap();
        let array_pattern_elem =
            ArrayPatternElem::from_pest(&mut pest::iterators::Pairs::single(pair)).unwrap();
        assert_eq!(
            array_pattern_elem,
            ArrayPatternElem::EclipsePattern(EclipsePattern {
                value: Id::NormalId(NormalId {
                    value: "a".to_string()
                }),
                diag: Diagnostic::test()
            })
        );
    }

    #[test]
    fn test_array_pattern_body() {
        let pair = SapParser::parse(Rule::array_pattern_body, "...a, ...b")
            .unwrap()
            .next()
            .unwrap();
        let array_pattern_body =
            ArrayPatternBody::from_pest(&mut pest::iterators::Pairs::single(pair)).unwrap();
        assert_eq!(array_pattern_body.elems.len(), 2);
        assert_eq!(
            array_pattern_body.elems[0],
            ArrayPatternElem::EclipsePattern(EclipsePattern {
                value: Id::NormalId(NormalId {
                    value: "a".to_string()
                }),
                diag: Diagnostic::test()
            })
        );
        assert_eq!(
            array_pattern_body.elems[1],
            ArrayPatternElem::EclipsePattern(EclipsePattern {
                value: Id::NormalId(NormalId {
                    value: "b".to_string()
                }),
                diag: Diagnostic::test()
            })
        );
    }

    #[test]
    fn test_array_pattern() {
        let pair = SapParser::parse(
            Rule::array_pattern,
            "^[...a, 
        ...b]",
        )
        .unwrap()
        .next()
        .unwrap();
        let array_pattern =
            ArrayPattern::from_pest(&mut pest::iterators::Pairs::single(pair)).unwrap();
        assert_eq!(array_pattern.body.elems.len(), 2);
        assert_eq!(
            array_pattern.body.elems[0],
            ArrayPatternElem::EclipsePattern(EclipsePattern {
                value: Id::NormalId(NormalId {
                    value: "a".to_string()
                }),
                diag: Diagnostic::test()
            })
        );
        assert_eq!(
            array_pattern.body.elems[1],
            ArrayPatternElem::EclipsePattern(EclipsePattern {
                value: Id::NormalId(NormalId {
                    value: "b".to_string()
                }),
                diag: Diagnostic::test()
            })
        );
    }
}
