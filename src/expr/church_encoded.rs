use super::*;

pub fn handle_expr_lift_c_params(expr: Expr, CParamsBody(params): CParamsBody) -> Expr {
    Expr::CApply(Box::new(expr), params.clone())
}

fn handle_mlapply(expr: Expr, pe: Box<Expr>) -> Expr {
    if let Expr::MLApply(c, p) = *pe {
        let mut np = vec![*c];
        np.extend(p);
        Expr::MLApply(Box::new(expr), np)
    } else {
        Expr::MLApply(Box::new(expr), vec![*pe])
    }
}

fn handle_infix(expr: Expr, pe: Box<Expr>) -> Option<Expr> {
    match *pe {
        Expr::Infix(i, box Expr::MLApply(c, p), ce) => {
            let mut np = vec![*c];
            np.extend(p);
            Some(Expr::Infix(
                i,
                Box::new(Expr::MLApply(Box::new(expr), np)),
                ce,
            ))
        }
        Expr::Infix(i, c, ce) => Some(Expr::Infix(
            i,
            Box::new(Expr::MLApply(Box::new(expr), vec![*c])),
            ce,
        )),
        _ => None,
    }
}

fn handle_postfix_trinary(expr: Expr, pe: Box<Expr>) -> Option<Expr> {
    match *pe {
        Expr::Postfix(Postfix::Trinary(t), box Expr::MLApply(c, p)) => {
            let mut np = vec![*c];
            np.extend(p);
            Some(Expr::Postfix(
                Postfix::Trinary(t),
                Box::new(Expr::MLApply(Box::new(expr), np)),
            ))
        }
        Expr::Postfix(Postfix::Trinary(t), c) => Some(Expr::Postfix(
            Postfix::Trinary(t),
            Box::new(Expr::MLApply(Box::new(expr), vec![*c])),
        )),
        _ => None,
    }
}

pub fn handle_expr_church_encoded(expr: Expr, MlAppParam(pe): MlAppParam) -> Expr {
    handle_postfix_trinary(expr.clone(), pe.clone())
        .or_else(|| handle_infix(expr.clone(), pe.clone()))
        .unwrap_or_else(|| handle_mlapply(expr, pe))
}
