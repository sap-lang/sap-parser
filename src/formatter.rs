pub enum FunctionApplicationStyle {
    CStyle,
    MLStyle,
}

pub struct GlobalOptions {
    pub indent_size: u32,
    pub screen_width: u32,

    pub function_application_style: FunctionApplicationStyle,

    /// transform small postfix_index `a["b"]` to `a.b`
    pub small_postfix_index_to_postfix_access: bool,

    /// transform small block `\x -> { x }` to `\x -> x`
    pub small_block_to_inline: bool,

    /// transform lambda with out parameter to no_parameter lambda
    /// `\->{ x }` to `_{ x }`
    pub lambda_no_parameter: bool,

    /// [1,2,3] -> [1,2,3,]
    /// {a:1,b:2} -> {a:1,b:2,}
    pub always_tailing_comma: bool,

    /// binary function call to infix
    /// `a b c` -> `b ~a~ c`
    pub binary_function_to_infix: bool,
}

pub struct Context {
    pub indent_level: u32,
}

pub trait PrettyPrint {
    fn pretty_print(&self, context: &Context, options: &GlobalOptions) -> String;
    /// for list, object, block, if, op,
    /// return None if it can not be multiline
    fn multiline_pretty_print(
        &self,
        _context: &Context,
        _options: &GlobalOptions,
    ) -> Option<String> {
        None
    }
}
