use pest_ast::FromPest;
use serde::Serialize;

use crate::{Rule, expr::Expr, id::Id, pattern::Pattern};

#[derive(Debug, Clone, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::implicit_params))]
pub struct ImplicitParams {
    pub params: Vec<Id>,
}

#[derive(Debug, Clone, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::guard))]
pub struct Guard {
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::tr_lambda))]
pub struct TrLambda {
    pub patterns: Vec<Pattern>,
    pub implicit_params: Option<ImplicitParams>,
    pub guard: Option<Guard>,
    pub body: Box<Expr>,
}

#[derive(Debug, Clone, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::no_param_lambda_expr))]
pub struct NoParamLambdaExpr {
    pub body: Vec<Expr>,
}

#[derive(Debug, Clone, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::lambda_expr))]
pub enum LambdaExpr {
    TrLambda(TrLambda),
    NoParamLambdaExpr(NoParamLambdaExpr),
}
