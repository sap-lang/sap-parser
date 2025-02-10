pub mod infix;
pub mod postfix;
pub mod prefix;

mod church_encoded;
pub use church_encoded::*;

use infix::Infix;

use pest_ast::FromPest;
use postfix::{CParamsBody, MlAppParam, Postfix, parse_postfix};
use prefix::Prefix;
use serde::Serialize;

use crate::{
    Rule, ast_with_diagnostic,
    function::LambdaExpr,
    id::Id,
    literal::CompoundLiteral,
    operator_precedence::pratt_parser,
    pattern::Pattern,
    quotations::{Block, ParenExpr},
};

ast_with_diagnostic! {
    Primary(primary) {
        Block(block: Block),
        LambdaExpr(lambda_expr: LambdaExpr),
        ParenExpr(paren_expr: ParenExpr),
        CompoundLiteral(compound_literal: CompoundLiteral),
        Id(id: Id),
        Pattern(pattern: Pattern),
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Expr {
    pub inner: ExprInner,
    pub diag: Diagnostic,
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
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
        let pest = pest.unwrap();

        let diag = Diagnostic::from_span(pest.as_span());
        let inner = ExprInner::from_pest(&mut Pairs::single(pest))?;
        Ok(Expr { inner, diag })
    }
}

impl Expr {
    #[allow(non_snake_case)]
    pub fn Prefix(prefix: Prefix, expr: Box<Expr>, diag: Diagnostic) -> Self {
        Expr {
            inner: ExprInner::Prefix(prefix, expr),
            diag,
        }
    }

    #[allow(non_snake_case)]
    pub fn Postfix(postfix: Postfix, expr: Box<Expr>, diag: Diagnostic) -> Self {
        Expr {
            inner: ExprInner::Postfix(postfix, expr),
            diag,
        }
    }

    #[allow(non_snake_case)]
    pub fn Infix(infix: Infix, lhs: Box<Expr>, rhs: Box<Expr>, diag: Diagnostic) -> Self {
        Expr {
            inner: ExprInner::Infix(infix, lhs, rhs),
            diag,
        }
    }

    #[allow(non_snake_case)]
    pub fn CApply(expr: Box<Expr>, params: Vec<Expr>, diag: Diagnostic) -> Self {
        Expr {
            inner: ExprInner::CApply(expr, params),
            diag,
        }
    }

    #[allow(non_snake_case)]
    pub fn MLApply(expr: Box<Expr>, params: Vec<Expr>, diag: Diagnostic) -> Self {
        Expr {
            inner: ExprInner::MLApply(expr, params),
            diag,
        }
    }

    #[allow(non_snake_case)]
    pub fn Primary(primary: Primary, diag: Diagnostic) -> Self {
        Expr {
            inner: ExprInner::Primary(primary),
            diag,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum ExprInner {
    Prefix(Prefix, Box<Expr>),
    Primary(Primary),
    Postfix(Postfix, Box<Expr>),
    Infix(Infix, Box<Expr>, Box<Expr>),
    // nano pass: remove church encoded ml_param
    CApply(Box<Expr>, Vec<Expr>),
    // nano pass: lift postfix c_params to expr
    MLApply(Box<Expr>, Vec<Expr>),
}

impl FromPest<'_> for ExprInner {
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
                let diag = primary.diag;
                Ok(Expr::Primary(primary, diag))
            })
            .map_prefix(|prefix, expr| {
                let span = prefix.as_span();
                let mut pairs = pest::iterators::Pairs::single(prefix);
                let diag = Diagnostic::from_span(span);
                let prefix = Prefix::from_pest(&mut pairs)?;
                let expr = expr?;
                let diag = diag.set_end_as(&expr.diag);
                Ok(Expr::Prefix(prefix, Box::new(expr), diag))
            })
            // find all PostfixExpr = ml_param which is church encoded
            // expand church encoded ml_param
            .map_postfix(|expr, postfix| {
                let expr = expr?;
                let span = postfix.as_span();
                let diag = Diagnostic::from_span(span);
                let diag = diag.set_start_as(&expr.diag);
                let postfix = parse_postfix(postfix)?;
                // expand church encoded ml_param
                if let Postfix::MlAppParam(p) = postfix {
                    Ok(handle_expr_church_encoded(expr, p))
                } else if let Postfix::CAppParams(p) = postfix {
                    if let Some(p) = p.0 {
                        Ok(handle_expr_lift_c_params(expr, p))
                    } else {
                        Ok(Expr::CApply(Box::new(expr), vec![], diag))
                    }
                } else {
                    Ok(Expr::Postfix(postfix, Box::new(expr), diag))
                }
            })
            .map_infix(|lhs, infix, rhs| {
                let lhs = lhs?;
                let span = infix.as_span();
                let mut pairs = pest::iterators::Pairs::single(infix);
                let diag = Diagnostic::from_span(span);
                let infix = Infix::from_pest(&mut pairs)?;
                let rhs = rhs?;
                let diag = diag.set_start_as(&lhs.diag);
                let diag = diag.set_end_as(&rhs.diag);
                Ok(Expr::Infix(infix, Box::new(lhs), Box::new(rhs), diag))
            })
            .parse(pest);

        expr.map(|expr| expr.inner)
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
    fn test_expr_capp() {
        let pair = crate::SapParser::parse(Rule::expr, "a(b,c)")
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
        let pair = crate::SapParser::parse(Rule::expr, "a b + c ? 1 : 2")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = pest::iterators::Pairs::single(pair);
        let expr = Expr::from_pest(&mut pairs).unwrap();

        println!("{:#?}", expr);
    }

    #[test]
    fn test_expr_fn() {
        let pair = crate::SapParser::parse(
            Rule::expr,
            "@@entry \
main = _{
    a = 1
    b = 2
    a |> b |> c
}",
        )
        .unwrap()
        .next()
        .unwrap();
        let mut pairs = pest::iterators::Pairs::single(pair);
        let expr = Expr::from_pest(&mut pairs).unwrap();

        println!("{:#?}", expr);
    }
}
