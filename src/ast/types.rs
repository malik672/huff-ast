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
    error_name,
    function_type_name,
    macro_type_name,
    constant(storage),
    event,
    jump_table,
    code_table
}

impl Node for type_name {
    fn accept(&self, definitions: &mut impl ast_definitions) -> Result<()> {

    }
}