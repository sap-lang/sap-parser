use pest_ast::FromPest;
use serde::Serialize;

use crate::Rule;

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
    pub body: ArrayPatternBody,
}

#[cfg(test)]
mod tests {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::{Rule, pattern::array::ArrayPatternElem};

    #[test]
    fn test_array_pattern_elem() {
        let pair = crate::SapParser::parse(Rule::array_pattern_elem, "...a")
            .unwrap()
            .next()
            .unwrap();
        let array_pattern_elem =
            ArrayPatternElem::from_pest(&mut pest::iterators::Pairs::single(pair)).unwrap();
        assert_eq!(
            array_pattern_elem,
            ArrayPatternElem::EclipsePattern(crate::pattern::EclipsePattern {
                value: crate::Id::NormalId(crate::NormalId {
                    value: "a".to_string()
                })
            })
        );
    }

    #[test]
    fn test_array_pattern_body() {
        let pair = crate::SapParser::parse(Rule::array_pattern_body, "...a, ...b")
            .unwrap()
            .next()
            .unwrap();
        let array_pattern_body = crate::pattern::array::ArrayPatternBody::from_pest(
            &mut pest::iterators::Pairs::single(pair),
        )
        .unwrap();
        assert_eq!(array_pattern_body.elems.len(), 2);
        assert_eq!(
            array_pattern_body.elems[0],
            ArrayPatternElem::EclipsePattern(crate::pattern::EclipsePattern {
                value: crate::Id::NormalId(crate::NormalId {
                    value: "a".to_string()
                })
            })
        );
        assert_eq!(
            array_pattern_body.elems[1],
            ArrayPatternElem::EclipsePattern(crate::pattern::EclipsePattern {
                value: crate::Id::NormalId(crate::NormalId {
                    value: "b".to_string()
                })
            })
        );
    }

    #[test]
    fn test_array_pattern() {
        let pair = crate::SapParser::parse(
            Rule::array_pattern,
            "^[...a, 
        ...b]",
        )
        .unwrap()
        .next()
        .unwrap();
        let array_pattern = crate::pattern::array::ArrayPattern::from_pest(
            &mut pest::iterators::Pairs::single(pair),
        )
        .unwrap();
        assert_eq!(array_pattern.body.elems.len(), 2);
        assert_eq!(
            array_pattern.body.elems[0],
            ArrayPatternElem::EclipsePattern(crate::pattern::EclipsePattern {
                value: crate::Id::NormalId(crate::NormalId {
                    value: "a".to_string()
                })
            })
        );
        assert_eq!(
            array_pattern.body.elems[1],
            ArrayPatternElem::EclipsePattern(crate::pattern::EclipsePattern {
                value: crate::Id::NormalId(crate::NormalId {
                    value: "b".to_string()
                })
            })
        );
    }
}
