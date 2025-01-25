use pest_ast::FromPest;
use serde::Serialize;

use crate::{
    Rule,
    function::LambdaExpr,
    id::Id,
    literal::CompoundLiteral,
    quotations::{Block, ParenExpr},
};

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::op_expr))]
pub enum OpExpr {
    Block(Block),
    LambdaExpr(LambdaExpr),
    ParenExpr(ParenExpr),
    CompoundLiteral(CompoundLiteral),
    Id(Id),
}

#[cfg(test)]
mod tests {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::{
        Rule,
        literal::number::{DecInt, Int},
    };

    #[test]
    fn test_op_id() {
        let pair = crate::SapParser::parse(Rule::op_expr, "a")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = pest::iterators::Pairs::single(pair);
        let op_expr = super::OpExpr::from_pest(&mut pairs).unwrap();
        assert_eq!(
            op_expr,
            super::OpExpr::Id(crate::id::Id::NormalId(crate::id::NormalId {
                value: "a".to_string()
            }))
        );
    }

    #[test]
    fn test_op_compound_literal() {
        let pair = crate::SapParser::parse(Rule::op_expr, "1")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = pest::iterators::Pairs::single(pair);
        let op_expr = super::OpExpr::from_pest(&mut pairs).unwrap();
        assert_eq!(
            op_expr,
            super::OpExpr::CompoundLiteral(crate::literal::CompoundLiteral::Literal(
                crate::literal::Literal::Number(crate::literal::number::SapNumber::Int(
                    Int::DecInt(DecInt { value: 1 })
                ))
            ))
        );
    }
}
