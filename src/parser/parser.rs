
pub struct ProgramNode {
    body: Vec<StatementNode>,
}

pub enum StatementNode {
    Include {
        path: StringLiteralNode,
    },

    Define {
        identifier: IdentifierNode,
        expression: ExpressionNode,
    },

    MacroCreate {
        identifier: IdentifierNode,
        parameters: Vec<IdentifierNode>,
        statements: Vec<StatementNode>,
    },

    LabelCreate {
        identifier: IdentifierNode,
    },

    Instruction {
        optcode: u64,
        argument: ExpressionNode,
    },

    ValueDefinition {
        value: ExpressionNode,
    },

    MacroCall {
        identifier: IdentifierNode,
        arguments: Vec<ExpressionNode>
    },
}

pub struct IdentifierNode(String);

pub enum ExpressionNode {
    Binary {
        a: Box<ExpressionNode>,
        b: Box<ExpressionNode>,
        operator: OperatorNode,
    },

    Literal(NumberLiteralNode),
    Identifier(IdentifierNode),
}

pub struct NumberLiteralNode(u64);
pub struct StringLiteralNode(String);

pub enum OperatorNode {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// program
//      include
//          Percent, Keyword(Include), StringLiteral([path])
//      define
//          Percent, Keyword(Define), Symbol([name]), ExpressionParser,
//      macro-create
//          Keyword(Macro), Symbol([name]), Until<Keyword(End), StatementParser>, Keyword(End)
//
//      [StatementParser]
//          label
//              Symbol([name]), Char(Colon)
//          instruction
//              Keyword(Instruction([value])) | Number([value]), ExpressionParser | expression
//          macro-call
//              Symbol([macro-name]), Char(Bang), While<Expression([argument]), Char(Comma)>

//  ExpressionParser
//      [ExpressionParser], Operation(_), [ExpressionParser]
//  |   Parentheses(Open), [ExpressionParser], Parentheses(Closed)
//  |   NumberLiteral(_)
//  |   Symbol(_)


// steps:
//      collect macros / defines
//      expand macros
//      expand defines
//      collect labels
//      make addresses from labels
//      replace symbols for labels with addresses
//      evaluate expressions
//
//      generate code 


// regexp-parser
//      [...tokens,]                token collection
//      (name:...tokens)            capture group
//      +, *                        quanitfiers
