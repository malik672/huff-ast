use eyre::Result;
use definitions::ast_definitions::*;

//Define type_descriptions in huff
pub struct type_descriptions {
    pub type_identifier: Option<String>,
    pub type_string: Option<String>,
    pub type_return: Option<String>,
    pub type_properties: Option<Vec<Sting>>,
    pub type_methods: Option<Vec<String>>
}

//Define custom types in huff
pub enum type_name {
    error_name(error_huff),
    function_type_name(function_huff),
    macro_type_name(macro_huff),
    constant(storage_huff),
    event(event_huff),
    jump_table(jump_huff),
    code_table(code_huff),
    String(string),
}

impl Node for type_name {
    fn accept(&self, definitions: &mut impl ast_definitions) -> Result<()> {
       match self {
        type_name::error_name(error_huff) => error_huff.accept(definitions),
        type_name::function_type_name(function_huff) => function_huff.accept(definitions),
        type_name::macro_type_name(macro_huff) => macro_huff.accept(definitions),
        type_name::constant(storage_huff) => storage_huff.accept(definitions),
        type_name::event(event_huff) => event_huff.accept(definitions),
        type_name::jump_table(jump_huff) => jump_huff.accept(definitions),
        type_name::code_table(code_huff) => code_huff.accept(definitions),
        type_name::String(_) => {
            panic!()
        }
       }
    }
}

impl Display for type_name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            type_name::error_name(error_huff) => error_huff.fmt(f),
            type_name::function_type_name(code_huff) => code_huff.fmt(f),
            type_name::macro_type_name(macro_huff) => macro_huff.fmt(f),
            type_name::constant(storage_huff) => storage_huff.fmt(f),
            type_name::event(event_huff) => event_huff.fmt(f),
            type_name::jump_table(jump_huff) => jump_huff.fmt(f),
            type_name::code_table(code_huff) => code_huff.fmt(f),
            type_name::String(string) => string.fmt(f), 
              _=> panic!("Invalid"),
        }
    }
}

pub struct function_type {
    pub visibility: Option<String>,
    pub f_type: Option<String>,
    pub param_type: Option<Vec<String>>,
    pub return_param: Option<Vec<String>>,
    pub f_descriptions: Option<String>,
    pub is_receive_ether: Option<String>
}

impl Node for function_type {
    fn accept(&self, f_type: &mut impl ast_definitions) -> Result<()> {
        if f_type.visit_function_type_name(self)? {
            self.param_type.accept(f_type)?;
            self.return_param.accept(f_type)?;
        }
        f_type.end_visit.function_type_name(self)
    }
}