use crate::tokenizer::{TokenType, Token, Keyword, Char, Brace, BracketType, Operator};

pub enum Expression {
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mult(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Value(i64),
    SymbolValue(String),
}

pub enum Instruction {
    Named(u64, String),                     // optcode, name
    Number(u64),                            // optcode
}

pub enum Statement {
    Label(String),                          // name of the label
    Instruction(Instruction, Expression),   // instruction, argument
    MacroCall(String, Vec<Expression>),     // name of macro, arguments for macro call
}

pub enum AstNode {
    Program(Vec<AstNode>),                  // children of program

    Include(String),                        // include path
    Define(String, Expression),             // define name, define value
    MacroCreate(String, Vec<Statement>),    // name of macro, macro body

    Statement(Statement),
    Expression(Expression),

    Collection(Vec<AstNode>)
}

pub enum ParserError {
    Default(String),
    UnbalancedBracketsError,
}

pub struct ParserResult {
    length: usize,
    node: AstNode,
}

pub trait Parser {
    fn digest(&self, tokens: &[TokenType]) -> Result<ParserResult, ParserError>;

    fn digest_all(&self, tokens: &[Token]) -> Result<AstNode, ParserError> {
        todo!()
    }
}

// match parser
macro_rules! match_parser {
    () => {
        
    };
}

// sequence parser
pub struct SequenceParserItem {
    pub parser: Box<dyn Parser>,
    pub keep: bool,
}
pub struct SequenceParser {
    pub parsers: Vec<SequenceParserItem>,
}
impl Parser for SequenceParser {
    fn digest(&self, tokens: &[TokenType]) -> Result<ParserResult, ParserError> {
        let mut idx = 0;
        let mut res_vec = Vec::new();

        for parser in &self.parsers {
            let res = parser.parser.digest(&tokens[idx..])?;

            if parser.keep { res_vec.push(res.node) }
            idx += res.length;
        }

        Ok(ParserResult {
            length: idx,
            node: AstNode::Collection(res_vec)
        })
    }
}

// program
//      include
//          Percent, Keyword(Include), StringLiteral([path])
//      define
//          Percent, Keyword(Define), Symbol([name]), ExpressionParser,
//      macro-create
//          Keyword(Macro), Symbol([name]), Char(Bang), Until<Keyword(End), StatementParser>, Keyword(End)
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
