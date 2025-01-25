use from_pest::FromPest;
use pest::iterators::{Pair, Pairs};
use pest_ast::FromPest;
use serde::Serialize;

use crate::{Rule, id::Id};

use super::Expr;

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::c_params_body))]
pub struct CParamsBody(pub Vec<Expr>);

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::postfix_c_app_params))]
pub struct CAppParams(pub Option<CParamsBody>);

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::postfix_index))]
pub struct Index {
    pub postfix_index: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::postfix_slice))]
pub struct Slice {
    pub start: Option<Box<Expr>>,
    pub end: Option<Box<Expr>>,
    pub step: Option<Box<Expr>>,
}

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::postfix_ml_app_param))]
pub struct MlAppParam(pub Box<Expr>);

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::postfix_access))]
pub struct Access {
    pub id: Id,
}

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::postfix_trinary_op))]
pub struct Trinary {
    pub true_expr: Box<Expr>,
    pub false_expr: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Postfix {
    Trinary(Trinary),
    Slice(Slice),
    Index(Index),
    Access(Access),
    // these two should not be used in the final AST
    CAppParams(CAppParams),
    MlAppParam(MlAppParam),
}

pub fn parse_postfix(
    rule: Pair<Rule>,
) -> Result<Postfix, from_pest::ConversionError<from_pest::Void>> {
    let rrule = rule.as_rule();
    let mut pairs = Pairs::single(rule);
    match rrule {
        Rule::postfix_trinary_op => Ok(Postfix::Trinary(Trinary::from_pest(&mut pairs)?)),
        Rule::postfix_slice => Ok(Postfix::Slice(Slice::from_pest(&mut pairs)?)),
        Rule::postfix_index => Ok(Postfix::Index(Index::from_pest(&mut pairs)?)),
        Rule::postfix_access => Ok(Postfix::Access(Access::from_pest(&mut pairs)?)),
        Rule::postfix_c_app_params => Ok(Postfix::CAppParams(CAppParams::from_pest(&mut pairs)?)),
        Rule::postfix_ml_app_param => Ok(Postfix::MlAppParam(MlAppParam::from_pest(&mut pairs)?)),

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::{
        Rule,
        expr::{Expr, Primary, op_expr::OpExpr},
        literal::{CompoundLiteral, Literal, number::SapNumber},
    };

    #[test]
    fn test_c_params_body() {
        let pair = crate::SapParser::parse(Rule::c_params_body, "1,2")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = pest::iterators::Pairs::single(pair);
        let c_params_body = super::CParamsBody::from_pest(&mut pairs).unwrap();
        assert_eq!(c_params_body.0.len(), 2);
        if let Expr::Primary(Primary::OpExpr(OpExpr::CompoundLiteral(CompoundLiteral::Literal(
            Literal::Number(SapNumber::Int(n)),
        )))) = &c_params_body.0[0]
        {
            assert_eq!(n.value(), 1);
        }
    }

    #[test]
    fn test_postfix_index() {
        let pair = crate::SapParser::parse(Rule::postfix_index, "[1]")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = pest::iterators::Pairs::single(pair);
        let postfix_index = super::Index::from_pest(&mut pairs).unwrap();
        if let Expr::Primary(Primary::OpExpr(OpExpr::CompoundLiteral(CompoundLiteral::Literal(
            Literal::Number(SapNumber::Int(n)),
        )))) = *postfix_index.postfix_index
        {
            assert_eq!(n.value(), 1);
        }
    }

    #[test]
    fn test_postfix_access() {
        let pair = crate::SapParser::parse(Rule::postfix_access, ".a")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = pest::iterators::Pairs::single(pair);
        let postfix_access = super::Access::from_pest(&mut pairs).unwrap();
        assert_eq!(postfix_access.id.value(), "a");
    }
}
