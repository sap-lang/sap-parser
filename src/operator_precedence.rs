use std::sync::OnceLock;

use pest::pratt_parser::{Assoc, Op, PrattParser};

use crate::Rule;

// precedence the higher the weaker
static PRATT_PARSER: OnceLock<PrattParser<Rule>> = OnceLock::new();

pub fn pratt_parser() -> &'static PrattParser<Rule> {
    PRATT_PARSER.get_or_init(|| {
        PrattParser::new()
            // finally
            .op(Op::prefix(Rule::prefix_annotative_macro_call))
            // level 18 assign related
            .op(Op::infix(Rule::infix_assign, Assoc::Right)
                | Op::infix(Rule::infix_set, Assoc::Right)
                | Op::infix(Rule::infix_assign_yield, Assoc::Right)
                | Op::infix(Rule::infix_assign_slot, Assoc::Right))
            // level 17 _ ? _ : _
            .op(Op::postfix(Rule::postfix_trinary_op))
            .op(Op::infix(Rule::infix_match_equals, Assoc::Right))
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
            // level 1 postfixes
            .op(Op::postfix(Rule::postfix_ml_app_param))
            .op(Op::postfix(Rule::postfix_slice)
                | Op::postfix(Rule::postfix_index)
                | Op::postfix(Rule::postfix_access)
                | Op::postfix(Rule::postfix_c_app_params))
    })
}
