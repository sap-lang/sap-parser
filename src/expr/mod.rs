pub mod infix;
pub mod op_expr;
pub mod postfix;
pub mod prefix;

mod church_encoded;
pub use church_encoded::*;

use from_pest::FromPest;
use infix::Infix;
use op_expr::OpExpr;
use pest_ast::FromPest;
use postfix::{CParamsBody, MlAppParam, Postfix, parse_postfix};
use prefix::Prefix;
use serde::Serialize;

use crate::{Rule, operator_precedence::pratt_parser, pattern::Pattern};

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::primary))]
pub enum Primary {
    OpExpr(OpExpr),
    Pattern(Pattern),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Expr {
    Prefix(Prefix, Box<Expr>),
    Primary(Primary),
    Postfix(Postfix, Box<Expr>),
    Infix(Infix, Box<Expr>, Box<Expr>),
    // nano pass: remove church encoded ml_param
    CApply(Box<Expr>, Vec<Expr>),
    // nano pass: lift postfix c_params to expr
    MLApply(Box<Expr>, Vec<Expr>),
}

impl FromPest<'_> for Expr {
    type Rule = Rule;

    type FatalError = from_pest::Void;

    fn from_pest(
        pest: &mut pest::iterators::Pairs<'_, Self::Rule>,
    ) -> Result<Self, from_pest::ConversionError<Self::FatalError>> {
        let pest = pest.next();
        if pest.is_none() {
            return Err(from_pest::ConversionError::NoMatch);
        }
        let pest = pest.unwrap().into_inner();

        let expr: Result<Expr, _> = pratt_parser()
            .map_primary(|primary| {
                let mut pairs = pest::iterators::Pairs::single(primary);
                let primary = Primary::from_pest(&mut pairs)?;
                Ok(Expr::Primary(primary))
            })
            .map_prefix(|prefix, expr| {
                let mut pairs = pest::iterators::Pairs::single(prefix);
                let prefix = Prefix::from_pest(&mut pairs)?;
                let expr = expr?;
                Ok(Expr::Prefix(prefix, Box::new(expr)))
            })
            // find all PostfixExpr = ml_param which is church encoded
            // expand church encoded ml_param
            .map_postfix(|expr, postfix| {
                let expr = expr?;
                let postfix = parse_postfix(postfix)?;
                println!("postfix: {:#?}\n\n", postfix);
                // expand church encoded ml_param
                if let Postfix::MlAppParam(p) = postfix {
                    Ok(handle_expr_church_encoded(expr, p))
                } else if let Postfix::CAppParams(p) = postfix {
                    Ok(handle_expr_lift_c_params(expr, p))
                } else {
                    Ok(Expr::Postfix(postfix, Box::new(expr)))
                }
            })
            .map_infix(|lhs, infix, rhs| {
                let lhs = lhs?;
                println!("ll: {:#?}\n\n", lhs);
                let mut pairs = pest::iterators::Pairs::single(infix);
                let infix = Infix::from_pest(&mut pairs)?;
                let rhs = rhs?;
                println!("rr: {:#?}\n\n", rhs);

                Ok(Expr::Infix(infix, Box::new(lhs), Box::new(rhs)))
            })
            .parse(pest);

        expr
    }
}

#[cfg(test)]
mod tests {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::{Rule, expr::Expr};

    #[test]
    fn test_expr() {
        let pair = crate::SapParser::parse(Rule::expr, "1 + 2 * 3")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = pest::iterators::Pairs::single(pair);
        let expr = Expr::from_pest(&mut pairs).unwrap();
        println!("{:#?}", expr);
    }

    #[test]
    fn test_expr_mlapp() {
        let pair = crate::SapParser::parse(Rule::expr, "a b")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = pest::iterators::Pairs::single(pair);
        let expr = Expr::from_pest(&mut pairs).unwrap();
        println!("{:#?}", expr);
    }
    #[test]
    fn test_expr_mlapp_infix() {
        let pair = crate::SapParser::parse(Rule::expr, "a b + a c + b c")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = pest::iterators::Pairs::single(pair);
        let expr = Expr::from_pest(&mut pairs).unwrap();
        // (a b c) + 1
        println!("{:#?}", expr);
    }
    #[test]
    fn test_expr_mlapp_trinary() {
        let pair = crate::SapParser::parse(Rule::expr, "a b c ? 1 : 2")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = pest::iterators::Pairs::single(pair);
        let expr = Expr::from_pest(&mut pairs).unwrap();
        // (a b c) ? 1 : 2
        println!("{:#?}", expr);
    }
    #[test]
    fn test_expr_mlapp_infix_and_trinary() {
        let pair = crate::SapParser::parse(Rule::expr, "a b + 1 ? 1 : 2")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = pest::iterators::Pairs::single(pair);
        let expr = Expr::from_pest(&mut pairs).unwrap();

        // FIXME: should be ((a b) + c)) ? 1 : 2
        println!("{:#?}", expr);
    }
}
