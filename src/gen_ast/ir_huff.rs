use serde_json::Value;


pub enum IRNode {
    Program(Vec<IRNode>),
    FunctionDeclaration(
        String,
        Vec<ParameterDeclaration>,
        Option<Payable>,
        Option<State>,
        Option<Visibility>,
        Option<String>,
        Vec<IRNode>,
    ),
    Expression(Expression),
    Typedeclaration(TypeDeclaration),
    ConstantDelaration(String, Expression),
    Statement(Statement),
}

//Since it's a type 
pub enum  TypeDeclaration {
    Uint,
    Int,
    Address,
    Bool,
    Bytes32,
    Bytes4,
    Bytes,
    String,                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           

}

pub enum State {
    Pure,
    View,
    None,
}

pub enum Visibility{
    Private,
    Public,
    External,
    Internal
}
pub enum Expression {
    BinaryOperation(Box<Expression>, String, Box<Expression>),
    FunctionCall(String, Vec<Expression>),
    Literal(Literal),
    VariableReference(String),
    ArrayAccess(Box<Expression>, Box<Expression>),
    StructAccess(Box<Expression>, String),
}

pub enum Payable {
    Payable,   //Indicates Payable 
    NonPayable,//Indicates Non Payable
}

pub enum Statement {
    IfStatement(Expression, Vec<IRNode>, Vec<IRNode>),
    WhileLoop(Expression, Vec<IRNode>),
    VariableAssignment(String, Expression),
    ArrayAssignment(Box<Expression>, Box<Expression>, Expression),
    StructAssignment(Box<Expression>, String, Expression),
    ReturnStatement(Option<Expression>),
}

pub enum Literal {
    StringLiteral(String),
    IntegerLiteral(i64),
    BooleanLiteral(bool),
    AddressLiteral(String), 
}

pub struct FieldDeclaration {
    name: String,
    ty: String,
}

pub struct ParameterDeclaration {
    name: String,
    ty: String,
}

pub struct Mapping {
    key: Value,
    value: Value,
}

