use super::*;

pub fn handle_expr_lift_c_params(expr: Expr, CParamsBody(params): CParamsBody) -> Expr {
    let Expr { inner: pe, diag } = expr;
    Expr::CApply(Box::new(Expr { inner: pe, diag }), params.clone(), diag)
}

fn handle_mlapply(expr: Expr, pe: Expr) -> Expr {
    let Expr { inner: pe, diag } = pe;
    if let ExprInner::MLApply(c, p) = pe {
        let mut np = vec![*c];
        np.extend(p);
        Expr::MLApply(Box::new(expr), np, diag)
    } else {
        Expr::MLApply(Box::new(expr), vec![Expr { inner: pe, diag }], diag)
    }
}

fn handle_infix(expr: Expr, pe: Expr) -> Option<Expr> {
    let Expr { inner: pe, diag } = pe;
    match pe {
        ExprInner::Infix(
            i,
            box Expr {
                inner: ExprInner::MLApply(c, p),
                diag: idiag,
            },
            ce,
        ) => {
            let mut np = vec![*c];
            np.extend(p);
            Some(Expr::Infix(
                i,
                Box::new(Expr::MLApply(Box::new(expr), np, idiag)),
                ce,
                diag,
            ))
        }
        ExprInner::Infix(i, c, ce) => Some(Expr::Infix(
            i,
            Box::new(Expr::MLApply(Box::new(expr), vec![*c], diag)),
            ce,
            diag,
        )),
        _ => None,
    }
}

fn handle_postfix_trinary(expr: Expr, pe: Expr) -> Option<Expr> {
    let Expr { inner: pe, diag } = pe;
    match pe {
        ExprInner::Postfix(
            Postfix::Trinary(t),
            box Expr {
                inner: ExprInner::MLApply(c, p),
                diag: idiag,
            },
        ) => {
            let mut np = vec![*c];
            np.extend(p);
            Some(Expr::Postfix(
                Postfix::Trinary(t),
                Box::new(Expr::MLApply(Box::new(expr), np, idiag)),
                idiag,
            ))
        }
        ExprInner::Postfix(Postfix::Trinary(t), c) => Some(Expr::Postfix(
            Postfix::Trinary(t),
            Box::new(handle_expr_church_encoded(expr, MlAppParam(c))),
            diag,
        )),
        _ => None,
    }
}

pub fn handle_expr_church_encoded(expr: Expr, MlAppParam(pe): MlAppParam) -> Expr {
    handle_postfix_trinary(expr.clone(), *pe.clone())
        .or_else(|| handle_infix(expr.clone(), *pe.clone()))
        .unwrap_or_else(|| handle_mlapply(expr, *pe))
}
