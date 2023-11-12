use ast::*;

pub trait ast_definitions {
  fn visit_block(&mut self, node: &Block) -> Result<bool> {
    self.visit_node(node)
  }

  fn end_visit_block(&mut self, node: &Block) -> Result<()> {
    self.end_visit_node(node)
  }

  fn visit_label(&mut self, node: &Label) -> Result<bool> {
    self.visit_node(node)
  }

  fn end_visit_label(&mut self, node: &Label) -> Result<()> {
    self.end_visit_node(node)
  }
  
  fn visit_identifier_path(&mut self, node: &IdentifierPath) -> Result<bool> {
    self.visit_node(node)
  }

  fn end_visit_identifier_path(&mut self, node: &IdentifierPath) -> Result<()> {
    self.end_visit_node(node)
  }

  fn visit_error(&mut self, node: &Error) -> Result<bool> {
    self.visit_node(node)
  }

  fn end_visit_error(&mut self, node: &Error) -> Result<()> {
    self.end_visit_node(node)
  }

  fn visit_structured_dcumentation(&mut self, node: &StructuredDumentation) -> Result<bool> {
     self.visit_node(node)
  }

  fn end_visit_structured_dcumentation(&mut self, node: &StructuredDocumentation) -> Result<()> {
    self.end_visit_node(node)
  }

  fn visit_block_documentation(&mut self, node: &BlockDocumentation) -> Result<bool> {
    self.visit_node(node)
  }

  fn end_visit_block_documentation(&mut self, node: &BlockDocumentation) -> Result<()> {
    self.end_visit_node(node)
  }

  fn visit_function_type_name(&mut self, node: &FunctionTypeName) -> Result<bool> {
    self.visit_node(node)
  }

  fn end_visit_node(&mut self, node: &FunctionTypeName) -> Result<()> {
    self.end_visit_node(node)
  }

  fn visit_function_call(&mut self, node: &FunctionCall) -> Result<bool> {
    self.visit_function_call(node)
  }

  fn end_visit_function_call(&mut self, node: &FunctionCall) -> Result<()> {
    self.end_visit_node(node)
  }

  fn visit_identifier(&mut self, node: &Identifier) -> Result<bool> {
    self.visit_node(node)
  }

  fn end_visit_identifier(&mut self, node: &Identifier) -> Result<()> {
    self.end_visit_node(node)
  }

  fn visit_expression(&mut self, node: &Expression) -> Result<bool> {
    self.visit_node(node)
  }

  fn end_visit_expression(&mut self, node: &Expression) -> Result<()> {
    self.end_visit_expression(node)
  }

  fn visit_param_type(&mut self, node: &ParamType) -> Result<bool> {
    self.visit_node(node)
  }

  fn end_visit_param_type(&mut self, node: &ParamType) -> Result<()> {
    self.end_visit_node(node)
  }

  fn visit_param_list(&mut self, node: &ParamList) -> Result<bool> {
    self.visit_node(node)
  }

  fn end_visit_param_list(&mut self, node: &ParamList) -> Result<()> {
    self.end_visit_node(node)
  }

  fn visit_revert_statement(&mut self, node: &RevertStatement) -> Result<bool> {
    self.visit_node(node)
  }

  fn end_visit_revert_statement(&mut self, node: &RevertStatement) -> Result<()> {
    self.end_visit_node(node)
  }

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