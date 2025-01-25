use pest_ast::FromPest;
use serde::Serialize;

use crate::{Rule, expr::Expr, id::Id};

use super::string::SapString;

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::object_key))]
pub enum ObjectKey {
    Id(Id),
    String(SapString),
}

impl ObjectKey {
    pub fn value(&self) -> String {
        match self {
            ObjectKey::Id(id) => id.value(),
            ObjectKey::String(string) => string.value(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::object_elem_kv))]
pub struct ObjectElemKv {
    pub key: ObjectKey,
    pub value: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::object_body))]
pub struct ObjectBody {
    pub body: Vec<ObjectElemKv>,
}

#[cfg(test)]
mod tests {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::Rule;

    #[test]
    fn test_object_key() {
        let pair = crate::SapParser::parse(Rule::object_key, "a")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = pest::iterators::Pairs::single(pair.clone());
        let object_key = super::ObjectKey::from_pest(&mut pairs).unwrap();
        assert_eq!(object_key.value(), "a");
    }

    #[test]
    fn test_object_elem_kv() {
        let pair = crate::SapParser::parse(Rule::object_elem_kv, "a: 1")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = pest::iterators::Pairs::single(pair);
        let object_elem_kv = super::ObjectElemKv::from_pest(&mut pairs).unwrap();
        assert_eq!(object_elem_kv.key.value(), "a");
    }

    #[test]
    fn test_object_elem_kv_only_k() {
        let pair = crate::SapParser::parse(Rule::object_elem_kv, "a")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = pest::iterators::Pairs::single(pair);
        let object_elem_kv = super::ObjectElemKv::from_pest(&mut pairs).unwrap();
        assert_eq!(object_elem_kv.key.value(), "a");
    }

    #[test]
    fn test_object_body() {
        let pair = crate::SapParser::parse(Rule::object_body, "a: 1, b")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = pest::iterators::Pairs::single(pair);
        let object_body = super::ObjectBody::from_pest(&mut pairs).unwrap();
        assert_eq!(object_body.body.len(), 2);
    }
}
