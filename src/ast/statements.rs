use super::*;
use eyre::Result;


pub enum Statement {
    RevertStatement(RevertStatement),
    Return(Return),
    Emit(Emit),
    VariableDeclaration(VariableDeclaration),
    Expression(Expression),
    UnhandledStatement {
        src: Option<String>,
        id: Option<i64>
    }
}

impl Node for Statement {
    fn accept(&self, definitions: &mut impl ast_definitions) -> Result<()> {
        match self{
            Statement::RevertStatement(revert_statement) => revert_statement.accept(definitions),
            Statement::Return(return_value) => return_value.accept(definitions),
            Statement::Emit(emit) => emit.accept(definitions),
            Statement::VariableDeclaration(variable_declaration) => variable_declaration.accept(definitions),
            Statement::Expression(expression) => expression.accept(definitions),
            Statement::UnhandledStatement{..} => {
                Ok(())
            }
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        Statement::VariableDeclaration(fst) => fst.fmt(f),
        Statement::Return(fst) => fst.fmt(f),
        Statement::Emit(fst) => fst.fmt(f),
        Statement::RevertStatement(fst) => fst.fmt(f),
        Statement::Expression(fst) => fst.fmt(f),
        Statement::UnhandledStatement
      }
    }
}