pub mod formatter;
pub mod preprocessor;

pub mod literal;
pub mod pattern;

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::macro_id))]
pub struct MacroId {
    #[pest_ast(outer(with(span_to_string)))]
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::normal_id))]
pub struct NormalId {
    #[pest_ast(outer(with(span_to_string)))]
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::magic_fn_id))]
pub struct MagicFnId {
    #[pest_ast(outer(with(span_to_string)))]
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, FromPest, Serialize)]
#[pest_ast(rule(Rule::id))]
pub enum Id {
    NormalId(NormalId),
    MacroId(MacroId),
    MagicFnId(MagicFnId),
}

impl Id {
    pub fn value(&self) -> String {
        match self {
            Id::NormalId(normal_id) => normal_id.value.clone(),
            Id::MacroId(macro_id) => macro_id.value.clone(),
            Id::MagicFnId(magic_fn_id) => magic_fn_id.value.clone(),
        }
    }
}

use std::sync::OnceLock;

use pest::pratt_parser::{Assoc, Op, PrattParser};
use pest_ast::FromPest;
use pest_derive::Parser;
use serde::Serialize;

#[derive(Parser)]
#[grammar = "parser.pest"]
pub struct SapParser;

pub fn span_to_string(span: pest::Span) -> String {
    span.as_str().to_string()
}

// precedence the higher the weaker
static PRATT_PARSER: OnceLock<PrattParser<Rule>> = OnceLock::new();

pub fn pratt_parser() -> &'static PrattParser<Rule> {
    PRATT_PARSER.get_or_init(|| {
        PrattParser::new()
            // level 18 assign related
            .op(Op::infix(Rule::infix_assign, Assoc::Right)
                | Op::infix(Rule::infix_assign_yield, Assoc::Right)
                | Op::infix(Rule::infix_assign_slot, Assoc::Right)
                | Op::infix(Rule::infix_match_equals, Assoc::Right))
            // level 17 _ ? _ : _
            .op(Op::postfix(Rule::postfix_trinary_op))
            // lelve 16 _ ($ |>) _
            .op(Op::infix(Rule::infix_pipe, Assoc::Left)
                | Op::infix(Rule::infix_find_and_call_with_this, Assoc::Left))
            // level 15 _ ~id~ _
            .op(Op::infix(Rule::infix_function, Assoc::Left))
            // level 14 <- _
            .op(Op::prefix(Rule::prefix_yield))
            // level 13 _ <: _
            .op(Op::infix(Rule::infix_extends, Assoc::Right))
            // level 12 _ || _
            .op(Op::infix(Rule::infix_or, Assoc::Left))
            // level 11 _ && _
            .op(Op::infix(Rule::infix_and, Assoc::Left))
            // level 10 _ | _
            .op(Op::infix(Rule::infix_bit_or, Assoc::Left))
            // level 9 _ ^ _
            .op(Op::infix(Rule::infix_bit_xor, Assoc::Left))
            // level 8 _ & _
            .op(Op::infix(Rule::infix_bit_and, Assoc::Left))
            // level 7 _ (== !=) _
            .op(Op::infix(Rule::infix_eq, Assoc::Left) | Op::infix(Rule::infix_neq, Assoc::Left))
            // level 6 _ (< > <= >=) _
            .op(Op::infix(Rule::infix_lt, Assoc::Left)
                | Op::infix(Rule::infix_gt, Assoc::Left)
                | Op::infix(Rule::infix_le, Assoc::Left)
                | Op::infix(Rule::infix_ge, Assoc::Left))
            // level 5 _ (<< >>) _
            .op(Op::infix(Rule::infix_bit_shift_l, Assoc::Left)
                | Op::infix(Rule::infix_bit_shift_r, Assoc::Left))
            // level 4 _ (+ -) _
            .op(Op::infix(Rule::infix_add, Assoc::Left) | Op::infix(Rule::infix_sub, Assoc::Left))
            // level3 _ (* / %) _
            .op(Op::infix(Rule::infix_mul, Assoc::Left)
                | Op::infix(Rule::infix_div, Assoc::Left)
                | Op::infix(Rule::infix_mod, Assoc::Left))
            // level 2 (- ! ~) _
            .op(Op::prefix(Rule::prefix_not)
                | Op::prefix(Rule::prefix_neg)
                | Op::prefix(Rule::prefix_bit_not))
            // level 1 _!
            .op(Op::postfix(Rule::postfix_bang))
    })
}
