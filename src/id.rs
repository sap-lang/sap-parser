use pest_ast::FromPest;
use serde::Serialize;

use crate::{Rule, span_to_string};

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
