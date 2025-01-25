use pest_ast::FromPest;
use serde::Serialize;

use crate::{Rule, expr::Expr};

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::paren_expr))]
pub struct ParenExpr {
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::block))]
pub struct Block {
    pub exprs: Vec<Expr>,
}
