enum IRNode {
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

enum Expression {
    BinaryOperation(Box<Expression>, String, Box<Expression>),
    FunctionCall(String, Vec<Expression>),
    Literal(Literal),
    VariableReference(String),
    ArrayAccess(Box<Expression>, Box<Expression>),
    StructAccess(Box<Expression>, String),
}

enum Statement {
    IfStatement(Expression, Vec<IRNode>, Vec<IRNode>),
    WhileLoop(Expression, Vec<IRNode>),
    VariableAssignment(String, Expression),
    ArrayAssignment(Box<Expression>, Box<Expression>, Expression),
    StructAssignment(Box<Expression>, String, Expression),
    ReturnStatement(Option<Expression>),
}

enum Literal {
    StringLiteral(String),
    IntegerLiteral(i64),
    BooleanLiteral(bool),
    AddressLiteral(Address), 
}

struct FieldDeclaration {
    name: String,
    ty: String,
}

struct ParameterDeclaration {
    name: String,
    ty: String,
}
