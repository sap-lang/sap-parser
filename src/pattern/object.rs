use pest_ast::FromPest;
use serde::Serialize;

use crate::{Rule, literal::object::ObjectKey};

use super::{EclipsePattern, Pattern};

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::object_pattern_kv))]
pub struct ObjectPatternKv {
    pub key: ObjectKey,
    pub value: Option<Pattern>,
}

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::object_pattern_elem))]
pub enum ObjectPatternElem {
    ObjectPatternKv(ObjectPatternKv),
    EclipsePattern(EclipsePattern),
}

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::object_pattern_body))]
pub struct ObjectPatternBody {
    pub body: Vec<ObjectPatternElem>,
}

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::object_pattern))]
pub struct ObjectPattern {
    pub body: ObjectPatternBody,
}

#[cfg(test)]
mod tests {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::{Rule, pattern::object::ObjectPatternKv};

    #[test]
    fn test_object_pattern_kv() {
        let pair = crate::SapParser::parse(Rule::object_pattern_kv, "a : 1")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = pest::iterators::Pairs::single(pair);
        let object_pattern_kv = ObjectPatternKv::from_pest(&mut pairs).unwrap();
        assert_eq!(object_pattern_kv.key.value(), "a");
    }

    #[test]
    fn test_object_pattern_elem() {
        let pair = crate::SapParser::parse(Rule::object_pattern_elem, "...b")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = pest::iterators::Pairs::single(pair);
        let object_pattern_elem = super::ObjectPatternElem::from_pest(&mut pairs).unwrap();
        if let super::ObjectPatternElem::EclipsePattern(eclipse_pattern) = object_pattern_elem {
            assert_eq!(eclipse_pattern.value.value(), "b");
        }
    }

    #[test]
    fn test_object_pattern_body() {
        let pair = crate::SapParser::parse(Rule::object_pattern_body, "a : 1, ...b")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = pest::iterators::Pairs::single(pair);
        let object_pattern_body = super::ObjectPatternBody::from_pest(&mut pairs).unwrap();
        assert_eq!(object_pattern_body.body.len(), 2);
    }
}
