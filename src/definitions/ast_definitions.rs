use ast::*;

pub trait ast_definitions {

}


pub trait Node {
    fn accept(&self, f_type: &mut impl ast_definitions) -> Result<()> {
        Ok(())
    }
}

pub fn list_accept(list: &Vec<impl Node>, f_type: &mut impl ast_definitions) -> Result<()> {
    for elem in list {
        elem.accept(f_type)?;
    }
    Ok(())
}