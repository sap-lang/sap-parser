use from_pest::FromPest;
use serde::Serialize;

use crate::{Rule, id::MacroId};

use super::Expr;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Prefix {
    Not,
    BitNot,
    Neg,
    Yield,
    AnnotativeMacroCall(MacroId, Option<Box<Expr>>),
}

impl FromPest<'_> for Prefix {
    type Rule = Rule;

    type FatalError = from_pest::Void;

    fn from_pest(
        pest: &mut pest::iterators::Pairs<'_, Self::Rule>,
    ) -> Result<Self, from_pest::ConversionError<Self::FatalError>> {
        let next = pest.next().unwrap();
        match next.as_rule() {
            Rule::prefix_not => Ok(Prefix::Not),
            Rule::prefix_bit_not => Ok(Prefix::BitNot),
            Rule::prefix_neg => Ok(Prefix::Neg),
            Rule::prefix_yield => Ok(Prefix::Yield),
            Rule::prefix_annotative_macro_call => {
                let mut pairs = next.into_inner();
                let macro_id = MacroId::from_pest(&mut pairs)?;
                let expr = pairs.next().map(|pair| {
                    Box::new(Expr::from_pest(&mut pest::iterators::Pairs::single(pair)).unwrap())
                });
                Ok(Prefix::AnnotativeMacroCall(macro_id, expr))
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::{
        Rule,
        expr::{Expr, prefix::Prefix},
        id::MacroId,
        literal::number::DecInt,
    };

    #[test]
    fn test_prefix_not() {
        let pair = crate::SapParser::parse(Rule::prefix_op, "!")
            .unwrap()
            .next()
            .unwrap();
        let prefix = Prefix::from_pest(&mut pest::iterators::Pairs::single(pair)).unwrap();
        assert_eq!(prefix, Prefix::Not);
    }

    #[test]
    fn test_prefix_bit_not() {
        let pair = crate::SapParser::parse(Rule::prefix_op, "~")
            .unwrap()
            .next()
            .unwrap();
        let prefix = Prefix::from_pest(&mut pest::iterators::Pairs::single(pair)).unwrap();
        assert_eq!(prefix, Prefix::BitNot);
    }

    #[test]
    fn test_prefix_neg() {
        let pair = crate::SapParser::parse(Rule::prefix_op, "-")
            .unwrap()
            .next()
            .unwrap();
        let prefix = Prefix::from_pest(&mut pest::iterators::Pairs::single(pair)).unwrap();
        assert_eq!(prefix, Prefix::Neg);
    }

    #[test]
    fn test_prefix_yield() {
        let pair = crate::SapParser::parse(Rule::prefix_op, "<-")
            .unwrap()
            .next()
            .unwrap();
        let prefix = Prefix::from_pest(&mut pest::iterators::Pairs::single(pair)).unwrap();
        assert_eq!(prefix, Prefix::Yield);
    }

    #[test]
    fn test_prefix_ann_macro_call() {
        let pair = crate::SapParser::parse(Rule::prefix_op, "@@macro (1)")
            .unwrap()
            .next()
            .unwrap();
        let prefix = Prefix::from_pest(&mut pest::iterators::Pairs::single(pair)).unwrap();
        assert_eq!(
            prefix,
            Prefix::AnnotativeMacroCall(
                MacroId {
                    value: "@macro".to_string()
                },
                Some(Box::new(Expr::Primary(crate::expr::Primary::OpExpr(
                    crate::expr::op_expr::OpExpr::CompoundLiteral(
                        crate::literal::CompoundLiteral::Literal(crate::literal::Literal::Number(
                            crate::literal::number::SapNumber::Int(
                                crate::literal::number::Int::DecInt(DecInt { value: 1 })
                            )
                        ))
                    )
                ))))
            )
        )
    }
}
