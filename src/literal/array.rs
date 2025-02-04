use pest_ast::FromPest;
use serde::Serialize;

use crate::{Rule, diagnostics::Diagnostic, expr::Expr};

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::array_body))]
pub struct ArrayBody {
    #[pest_ast(outer(with(Diagnostic::from_span)))]
    pub diag: Diagnostic,
    pub elems: Vec<Expr>,
}

#[cfg(test)]
mod tests {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::Rule;

    #[test]
    fn test_array_body() {
        let pair = crate::SapParser::parse(Rule::array_body, "1, 2, 3")
            .unwrap()
            .next()
            .unwrap();
        let mut pairs = pest::iterators::Pairs::single(pair);
        let array_body = super::ArrayBody::from_pest(&mut pairs).unwrap();
        assert_eq!(array_body.elems.len(), 3);
    }
}
