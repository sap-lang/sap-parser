use from_pest::FromPest;
use serde::Serialize;

use crate::{Rule, id::Id};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Infix {
    Add,
    AssignYield,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Extends,
    Le,
    Ge,
    Lt,
    Gt,
    And,
    Pipe,
    FindAndCallWithThis,
    Or,
    BitOr,
    BitAnd,
    BitXor,
    BitShiftL,
    BitShiftR,
    Function(Id),
    Assign,
    MatchEquals,
    AssignSlot,
}

impl FromPest<'_> for Infix {
    type Rule = Rule;

    type FatalError = from_pest::Void;

    fn from_pest(
        pest: &mut pest::iterators::Pairs<'_, Self::Rule>,
    ) -> Result<Self, from_pest::ConversionError<Self::FatalError>> {
        let next = pest.next().unwrap();
        match next.as_rule() {
            Rule::infix_add => Ok(Infix::Add),
            Rule::infix_assign_yield => Ok(Infix::AssignYield),
            Rule::infix_sub => Ok(Infix::Sub),
            Rule::infix_mul => Ok(Infix::Mul),
            Rule::infix_div => Ok(Infix::Div),
            Rule::infix_mod => Ok(Infix::Mod),
            Rule::infix_eq => Ok(Infix::Eq),
            Rule::infix_neq => Ok(Infix::Neq),
            Rule::infix_extends => Ok(Infix::Extends),
            Rule::infix_le => Ok(Infix::Le),
            Rule::infix_ge => Ok(Infix::Ge),
            Rule::infix_lt => Ok(Infix::Lt),
            Rule::infix_gt => Ok(Infix::Gt),
            Rule::infix_and => Ok(Infix::And),
            Rule::infix_pipe => Ok(Infix::Pipe),
            Rule::infix_find_and_call_with_this => Ok(Infix::FindAndCallWithThis),
            Rule::infix_or => Ok(Infix::Or),
            Rule::infix_bit_or => Ok(Infix::BitOr),
            Rule::infix_bit_and => Ok(Infix::BitAnd),
            Rule::infix_bit_xor => Ok(Infix::BitXor),
            Rule::infix_bit_shift_l => Ok(Infix::BitShiftL),
            Rule::infix_bit_shift_r => Ok(Infix::BitShiftR),
            Rule::infix_function => {
                let id = Id::from_pest(&mut next.into_inner())?;
                Ok(Infix::Function(id))
            }
            Rule::infix_assign => Ok(Infix::Assign),
            Rule::infix_match_equals => Ok(Infix::MatchEquals),
            Rule::infix_assign_slot => Ok(Infix::AssignSlot),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::{
        Rule, SapParser,
        expr::infix::Infix,
        id::{Id, NormalId},
    };

    #[test]
    fn test_infix_add() {
        let pair = SapParser::parse(Rule::infix_op, "+")
            .unwrap()
            .next()
            .unwrap();
        let infix = Infix::from_pest(&mut pest::iterators::Pairs::single(pair)).unwrap();
        assert_eq!(infix, Infix::Add);
    }

    #[test]
    fn test_infix_function() {
        let pair = SapParser::parse(Rule::infix_op, "~id~")
            .unwrap()
            .next()
            .unwrap();
        let infix = Infix::from_pest(&mut pest::iterators::Pairs::single(pair)).unwrap();
        assert_eq!(
            infix,
            Infix::Function(Id::NormalId(NormalId {
                value: "id".to_string()
            }))
        );
    }
}
