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
    code_table(code_huff)
}

impl Node for type_name {
    fn accept(&self, definitions: &mut impl ast_definitions) -> Result<()> {
       match self {
        type_name::error_name(error_huff) => error_huff.accept(ast_definitions),
        type_name::function_type_name(function_huff) => function_huff.accept(ast_definitions),
        type_name::macro_type_name(macro_huff) => macro_huff.accept(ast_definitions),
        type_name::constant(storage_huff) => storage_huff.accept(ast_definitions),
        type_name::event(event_huff) => event_huff.accept(ast_definitions),
        type_name::jump_table(jump_huff) => jump_huff.accept(ast_definitions),
        type_name::code_table(code_huff) => code_huff.accept(ast_definitions)

       }
    }
}